use libsqlite3_sys::*;

#[derive(PartialEq)]
pub struct ExtendedDatabaseErrorInfo {
    pub errcode: i32,
    pub extended_errcode: i32,
    pub extended_errstr: Option<String>,
    pub extended_err_offset:i32,
    // pub errmsg: Option<String>

}
impl ExtendedDatabaseErrorInfo {
    pub fn from_database_handle(db: &self::ConnectionHandle) -> Self {
        let extended_errstr = unsafe {
            let sqlite3_error_msg_ptr = libsqlite3_sys::sqlite3_errmsg(db.0);
            if sqlite3_error_msg_ptr.is_null() {
                None
            }
            else {Some(std::ffi::CStr::from_ptr(sqlite3_error_msg_ptr).to_string_lossy().into_owned())}
        };
        let errcode = unsafe {
            sqlite3_errcode(db.0)
        };
        let extended_errcode = unsafe {
            sqlite3_extended_errcode(db.0)
        };
        let extended_err_offset = unsafe {
            sqlite3_error_offset(db.0)
        };
        Self {
            errcode,
            extended_errcode,
            extended_err_offset,
            extended_errstr
        }
    }
}


#[repr(transparent)]
/// do not SYNC conns between threads, do not SHARE conns between workers only SEND them to the worker thread;
/// it is UNSAFE to attempt to share this type between threads. as such it does NOT and WILL NOT implement clone or copy.
/// if you abuse the deref and deref mut implementation to do something nasty, Author CANNOT GUARENTEE that it will be safe.
/// unsafe impl SEND only allows it to be SENT to the worker thread it was meant for
pub(crate) struct ConnectionHandle(*mut sqlite3);
impl ConnectionHandle {
    pub(crate) fn last_error(&self) -> ExtendedDatabaseErrorInfo {
        debug_assert_eq!(self.0.is_null(),false);
        ExtendedDatabaseErrorInfo::from_database_handle(&self)
    }


    pub(crate) fn connect<'url>(url: &'url std::ffi::CStr ) -> std::io::Result<Self> {
        let mut conn_ptr: *mut sqlite3 = std::ptr::null_mut();
        let result = unsafe {
            sqlite3_open_v2(
                url.as_ptr(),
                &mut conn_ptr,
                SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
                core::ptr::null(),
            )
        };
        if result != SQLITE_OK {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                format!(
                    "Failed to connect to sqlite DB. sqlite returned error code {result} in call to sqlite3_v2_open"
                ),
            ));
        }
        Ok(Self(conn_ptr))
    }
    pub(crate) fn get_autocommit(&self) -> i32 {
        unsafe {libsqlite3_sys::sqlite3_get_autocommit(self.0)}
    }
    /// caller must guarentee that the s
    pub(crate) fn execute_unpreapred_cstr<'conn, 'sql>(&'conn mut self,s: &'sql std::ffi::CStr) -> Result<(),String>{
        todo!()
    }
    pub (crate) fn execute_unprepared<'conn,'sql>(&'conn mut self,s: &'sql str) -> Result<(),String> {
        let cstr = std::ffi::CString::new(s).
        map_err(|e|format!("NulError: Theres a snake in my boot! \
        You put a nullpointer in your sql when trying to ConnectionHandle.execute_unprepared()!\n{e}").to_string())?;
        self.execute_unpreapred_cstr(&cstr)
        // todo!()

    }
    pub (crate) fn prepare<'conn,'sql>(&'conn self,sql: &str) -> super::types::Statement<'conn> {
        super::types::Statement::prepare(&self, sql).unwrap()
    }
    pub(crate) fn get_mut_for_stmt(&self,s: &super::types::Statement) -> *mut sqlite3 {
        self.0
    }
}
impl std::ops::Deref for ConnectionHandle {
    type Target = *mut sqlite3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// impl DerefMut for ConnectionHandle {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
unsafe impl Send for ConnectionHandle {}
unsafe impl Sync for ConnectionHandle{}

// WARNING: DROP MUST BE CALLED BEFORE WORKER is dropped.
// as long as connectionhandle goes out of scope when the worker exits
// you should be fine

impl Drop for ConnectionHandle {
    fn drop(&mut self) {
        // connection handle should never be null on drop
        debug_assert!(self.0.is_null() == false);
        unsafe {
            // attempt to call interrupt to get sqlite3 to stop whatever its doing. our connection is geting closed
            sqlite3_interrupt(self.0);

            let r = crate::drivers::sqlite::sqlite3_close_v2(self.0);
            // let r = sqlite3_close(self.0);
            if r != SQLITE_OK {
                eprintln!(
                    "Call to sqlite3_close failed on implicit drop for ConnectionHandle: sqlite3_close returned: {r}"
                )
            }
        }
    }
}