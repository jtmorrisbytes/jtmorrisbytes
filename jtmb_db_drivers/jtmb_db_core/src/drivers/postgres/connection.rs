use std::{net::TcpStream, os::windows::io::FromRawSocket};

use pq_sys::*;

pub enum PQClientMessage {}
pub enum PQClientResponse {}

pub struct ConnInfoOptions(*mut _PQconninfoOption);
impl Drop for ConnInfoOptions {
    fn drop(&mut self) {
        assert_eq!(self.0.is_null(), false);
        unsafe {
            PQconninfoFree(self.0);
        }
    }
}
unsafe impl Send for ConnInfoOptions {}
pub struct Connection(*mut PGconn);
// THE HANDSHAKE: "I promise I'm not sharing this pointer between threads
// simultaneously, only moving it."
unsafe impl Send for Connection {}
// If you need it in a static or shared State, you might need this too:
unsafe impl Sync for Connection {}
impl Connection {
    fn new(p: *mut PGconn) -> Self {
        debug_assert_eq!(p.is_null(), false);
        Self(p)
    }
    /// this function must be called during async comms to get the conn ready until you recive CONNECTION_OK
    /// does not 'block', but instead coops with tokio to park the task until the socket is ready
    /// to be used. returns an error if it fails
    async fn drive_connection_handshake_to_completion(
        &mut self,
        async_stream: &mut tokio::net::TcpStream,
    ) -> std::io::Result<()> {
        loop {
            let poll_status = unsafe { PQconnectPoll(self.0) };
            // let mut poll_status = PostgresPollingStatusType::PGRES_POLLING_WRITING;
            // this loop drives the socket
            match poll_status {
                PostgresPollingStatusType::PGRES_POLLING_READING => {
                    async_stream.readable().await?;
                }
                PostgresPollingStatusType::PGRES_POLLING_WRITING => {
                    async_stream.writable().await?;
                }
                PostgresPollingStatusType::PGRES_POLLING_ACTIVE => {
                    tokio::task::yield_now().await;
                }
                PostgresPollingStatusType::PGRES_POLLING_OK => return Ok(()),
                PostgresPollingStatusType::PGRES_POLLING_FAILED => {
                    let pg_message = self.pq_err_messaage();
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::ConnectionAborted,
                        format!(
                            "Failed to connect to database, libpq failed to poll the connection. libpq message: {pg_message}"
                        ),
                    ));
                }
            }
        }
    }
    pub fn status(&self) -> ConnStatusType {
        assert_eq!(self.0.is_null(), false);
        unsafe { PQstatus(self.0) }
    }
    pub fn pq_err_messaage(&self) -> String {
        let msg = unsafe { pq_sys::PQerrorMessage(self.0) };
        if msg.is_null() {
            return String::new();
        }
        unsafe { std::ffi::CStr::from_ptr(msg) }
            .to_string_lossy()
            .into_owned()
    }
    pub fn set_nonblocking(&self, block: bool) -> std::io::Result<()> {
        let result = unsafe { pq_sys::PQsetnonblocking(self.0, block as i32) };

        if result == -1 {
            // This usually means the connection is already "BAD"
            // or the 'Snake' is in the internal C state.
            let msg = self.pq_err_messaage();
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to set non_blocking on connection: libpq message: {msg}"),
            ));
        }
        Ok(())
    }
    // drives the nework socket if there is any data to be sent. this must be called after queing any data
    async fn flush(
        &mut self,
        async_stream: &mut tokio::net::TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 2. THE BLOW: Ensure the outgoing pipe is empty
        // PQflush returns 1 if it still has bytes to send
        let mut flush_status = unsafe { pq_sys::PQflush(self.0) };
        while flush_status == 1 {
            // Wait for the OS/Windows to say "I have room for more"
            async_stream.writable().await?;
            flush_status = unsafe { pq_sys::PQflush(self.0) };
        }

        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        debug_assert_eq!(self.0.is_null(), false);
        unsafe {
            PQfinish(self.0);
        }
    }
}
// never send this connection between threads or attempt to CLONE or COPY,
// instead use our parallel parsing and execution primitives to speed up your work
// and use multiple conns if required between actors

pub fn connect(
    url_or_paramstring: &str,
) -> std::io::Result<(
    tokio::sync::mpsc::Sender<PQClientMessage>,
    impl Future<Output = std::io::Result<()>>,
)> {
    // aquire and parse connection parameters
    let conn_string = url_or_paramstring.to_owned();
    let conn_cstring = std::ffi::CString::new(conn_string.as_str())
    .map_err(|_e|
        std::io::Error::new(std::io::ErrorKind::InvalidInput,
            format!("Unexpected \\0 nul byte in '{conn_string}'. do not pass null term'd strings into connect"))
    )?;

    let mut err_msg_ptr = std::ptr::null_mut();
    let conn_info = unsafe { pq_sys::PQconninfoParse(conn_cstring.as_ptr(), &mut err_msg_ptr) };
    if err_msg_ptr.is_null() == false {
        let err_msg = unsafe { std::ffi::CStr::from_ptr(err_msg_ptr) }
            .to_string_lossy()
            .to_string();
        return Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg));
    }
    let mut conn_info = ConnInfoOptions(conn_info);

    //YUCK! WHY? but ok...
    // 1. Prepare parallel arrays for libpq
    let mut keywords = Vec::new();
    let mut values = Vec::new();

    let mut current = conn_info.0; // Your *mut PQconninfoOption
    unsafe {
        while !(*current).keyword.is_null() {
            if !(*current).val.is_null() {
                keywords.push((*current).keyword);
                values.push((*current).val);
            }
            current = current.add(1);
        }
    }

    // Null-terminate the arrays as required by C FFI
    keywords.push(std::ptr::null_mut());
    values.push(std::ptr::null_mut());

    // actually perform the connection

    let conn_ptr = unsafe {
        pq_sys::PQconnectStartParams(
            keywords.as_ptr().cast(),
            values.as_ptr().cast(),
            0, // expand_dbname: 0 means don't treat dbname as a connstring
        )
    };
    if conn_ptr.is_null() {
        return Err(std::io::Error::from(std::io::ErrorKind::OutOfMemory));
    }

    // give the task a handle to revieve messages from the outside works
    let (task_sender, mut task_reciever) = tokio::sync::mpsc::channel::<PQClientMessage>(100);
    // the task 'engine
    let conn = Connection::new(conn_ptr);
    let fut = async move {
        let mut conn = conn;
        // check the status of the connection

        use std::os::windows::io::{AsRawSocket, RawSocket};

        // 1. Get the raw handle from libpq
        let raw_socket_handle = unsafe { pq_sys::PQsocket(conn.0) };

        // 2. On Win64, this is a 64-bit SOCKET, but libpq returns int (32-bit).
        // This truncation is a known libpq quirk on Windows.
        let socket_handle = raw_socket_handle as RawSocket;

        // This takes ownership of the socket.
        // BE CAREFUL: libpq also thinks it owns this socket.
        let std_stream = unsafe { std::net::TcpStream::from_raw_socket(socket_handle) };
        let mut async_stream = tokio::net::TcpStream::from_std(std_stream)?;

        // drive the connection process. you MUST call this function when making a connection or the socket will not be ready
        conn.drive_connection_handshake_to_completion(&mut async_stream)
            .await?;
        let status = unsafe { PQstatus(conn.0) };
        match status {
            ConnStatusType::CONNECTION_BAD => {
                let err_msg = conn.pq_err_messaage();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    err_msg.to_string(),
                ));
            }
            ConnStatusType::CONNECTION_OK => {}
            _ => {
                dbg!(status);
                todo!()
            }
        }

        loop {
            // first we check the status of the connection
            let status = conn.status();
            match status {
                ConnStatusType::CONNECTION_OK => {
                    println!("connection ok. continuing");
                }
                ConnStatusType::CONNECTION_BAD => {
                    let raw_msg = unsafe { pq_sys::PQerrorMessage(conn.0) };
                    let err_msg = unsafe { std::ffi::CStr::from_ptr(raw_msg).to_string_lossy() };
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::ConnectionAborted,
                        err_msg.to_string(),
                    ));
                }
                _ => {
                    dbg!(status);
                    todo!("conn status in main run loop")
                }
            }

            // now we probably need to drive the socket
            // flush any data down the pipe. you MUST call this fn after sending data but\
            // is included here for completeness
            conn.flush(&mut async_stream).await.ok();

            tokio::select! {
                // CASE A: A new "Nuke" task arrived from Rocket
                // This is a "biased" select, so we check tasks first
                biased;

                msg = task_reciever.recv() => {
                    match msg {
                        Some(m) => {
                            // handle_message(m, &mut conn, &mut async_stream).await?;
                        }
                        None => return Ok(()), // Channel closed, shut down the woodchipper
                    }
                }

                // CASE B: Postgres has bytes ready for the woodchipper
                _ = async_stream.readable() => {
                    unsafe { pq_sys::PQconsumeInput(conn.0) };
                    // while !PQisBusy { PQgetResult... blit... }
                }
            }

            break;
        }

        std::io::Result::Ok(())
    };
    // ok now we are ready to drive the machine

    Ok((task_sender, fut))
}

#[tokio::test]
pub async fn test_pg() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, task) = connect("postgres://postgres:postgres@localhost:5432/postgres")?;

    let handle = tokio::task::spawn(task);

    // you must await the task in a background task
    // let t_h = tokio::task::spawn_local(t)

    handle.await.ok();
    Ok(())
}
