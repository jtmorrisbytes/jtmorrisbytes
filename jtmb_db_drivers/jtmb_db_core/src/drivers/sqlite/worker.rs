use std::{sync::Arc, thread::JoinHandle};

use libsqlite3_sys::*;

use crate::drivers::sqlite::{ConnectOptions,connection::ConnectionHandle, types::StepResult};

#[derive(Debug)]
pub enum WorkerResponse {
    PrepareFailed(String),
    Schema(Vec<(String, i32)>),
    Row(Vec<u8>),
    Done,
}

pub enum WorkerTask {
    Execute(String, std::sync::mpsc::Sender<WorkerResponse>),
    Shutdown,
}

/// DROP must be called after the connection exits but before the driver gets dropped

impl Drop for Worker {
    fn drop(&mut self) {
        // attempt to send the message to the thread
        let handle = self
            .thread_handle
            .take()
            .expect("BUG BUG! Worker thread should be initialized");

        handle.thread().unpark();

        match self.sender.send(WorkerTask::Shutdown) {
            Err(e) => {
                println!(
                    "WARNING: failed to send shutdown signal to worker thread: {e}. connection may dangle."
                );
            }
            _ => {}
        }
        let r = handle.join();
        // println!("thread joined with result {r:?}");
    }
}

pub(crate) unsafe fn worker_execute(
    stmt: &mut super::types::Statement,
    sender: std::sync::mpsc::Sender<WorkerResponse>,
    row_data: &mut Vec<u8>,
) -> std::io::Result<()> {
    let rc = stmt.step();
    let mut rc: StepResult = match rc {
        StepResult::Error(_) | StepResult::Interrupted => {
            stmt.reset();
            return Err(std::io::Error::new(std::io::ErrorKind::Interrupted,"step failed".to_string()));
    },
        o=>{o}
    };
    // let num_columns = sqlite3_column_count(**stmt);
    let num_columns = stmt.column_count();
    // let mut rc = sqlite3_step(stmt);

    let mut metadata = Vec::new();
    for i in 0..num_columns {
        let col_type = unsafe { sqlite3_column_type(**stmt, i) };
        // dbg!(col_type);
        let n = unsafe {
            let name_ptr = unsafe { sqlite3_column_name(**stmt, i as i32) };
            if name_ptr.is_null() {
                metadata.push((String::new(), col_type));
            }
            std::ffi::CStr::from_ptr(name_ptr)
                .to_string_lossy()
                .to_string()
        };
        metadata.push((n, col_type))
    }
    sender.send(WorkerResponse::Schema(metadata.clone())).ok();
    let mut row_data = Vec::<u8>::with_capacity(256);
    while rc == StepResult::Row {
        for c in 0..num_columns {
            // fast vtble insead of column match
            unsafe {
                super::vtbl::EXTRACT_TBL[stmt.column_type(c) as usize](
                    **stmt,
                    c as i32,
                    &mut row_data,
                )
            }
        }
        rc = stmt.step();
        // println!("sqlite3_step {rc}");
    }
    match rc {
        StepResult::Interrupted => {
            stmt.reset();
            return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Execute was interrupted while calling step"))
        }
        StepResult::Error(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error))
        }
    }
    if rc != SQLITE_DONE {
        println!("Sqlite3 step returned a non DONE response {rc}");
    }
    // stmt.finalize();
    stmt.reset();
    // let r = sqlite3_finalize(**stmt);
    // println!("sqlite3_finalize {r}");
    sender.send(WorkerResponse::Done).ok();
    Ok(())
}

#[derive(Debug)]
pub struct Worker {
    // connection: *mut sqlite3,
    connection_options: ConnectOptions,
    // status: Arc<AtomicU32>,
    sender: std::sync::mpsc::Sender<WorkerTask>,
    thread_handle: Option<std::thread::JoinHandle<Result<(), std::io::Error>>>,
}
impl Worker {
    pub(crate) fn send(
        &self,
        task: WorkerTask,
    ) -> Result<(), std::sync::mpsc::SendError<WorkerTask>> {
        self.sender.send(task)
    }

    pub(crate) fn spawn(options: &ConnectOptions) -> std::io::Result<Self> {
        let url = std::ffi::CString::new(options.url.as_str())?;
        let (task_sender, task_reviever) = std::sync::mpsc::channel();
        
   
        
        let conn = ConnectionHandle::connect(&url)?;
        let conn = Arc::new(conn);
        // let status = Arc::new(AtomicU32::new(u32::MAX));

        // let status1 = status.clone();
        let thread_handle: JoinHandle<std::io::Result<()>> = std::thread::spawn(move || {
            let conn = conn;
            // let status = status1;
            // println!("Worker thread spawned");
            loop {

                let m = match task_reviever.recv() {
                    Err(e) => {
                        // sender disconnected. thread may be dropped
                        println!("Warning: Worker thread detected that task sender disconnected");
                        // todo!();
                        break;
                        // Driver::try_disconnect_on_worker_sender_dropped(conn_ptr);
                    }
                    // if the queue is empty, the thread will park
                    Ok(m) => m,
                };
                match m {
                    WorkerTask::Execute(sql, sender) => {
                        let mut stmt =
                            conn.prepare(&sql);
                        let mut row_data = Vec::with_capacity(128);
                        let r = unsafe { worker_execute(&mut stmt, sender, &mut row_data) };
                        match r {
                            Err(e) => {
                                eprintln!("{e}");
                                                // 1. Check if the previous task left the transaction open

                                
                            }
                            _ => {}
                        }
                        row_data.clear();
                                                        if unsafe { sqlite3_get_autocommit(**conn) } == 0 {
                                    // 2. Clear the poisoned state
                                    let _ = unsafe {
                                        sqlite3_exec(**conn, "ROLLBACK;\0".as_ptr().cast(), None, std::ptr::null_mut(), std::ptr::null_mut());
                                    };
                                }
                    }
                    WorkerTask::Shutdown => {
                        // println!("Shutdown requested");
                        break;
                    }
                }
            }
            std::io::Result::Ok(())
        });
        let s = Self {
            connection_options: options.to_owned(),
            sender: task_sender,
            thread_handle: Some(thread_handle),
            // status: status,
        };
        Ok(s)
    }
}
