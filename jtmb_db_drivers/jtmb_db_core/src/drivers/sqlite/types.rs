use libsqlite3_sys::{
    SQLITE_BUSY, SQLITE_DONE, SQLITE_INTERRUPT, SQLITE_OK, SQLITE_ROW, sqlite3_bind_double, sqlite3_bind_int64, sqlite3_bind_text, sqlite3_blob, sqlite3_column_blob, sqlite3_column_bytes, sqlite3_column_count, sqlite3_column_double, sqlite3_column_int64, sqlite3_column_text, sqlite3_column_type, sqlite3_destructor_type, sqlite3_finalize, sqlite3_prepare_v2, sqlite3_reset, sqlite3_step, sqlite3_stmt
};
use std::{ffi::CString, ops::Deref, sync::Arc};

use crate::drivers::sqlite::connection::{ConnectionHandle, ExtendedDatabaseErrorInfo};
pub type Real = f64;
pub type Float = f64;
pub type Integer = i64;
pub type Text = String;
struct Blob(Vec<u8>);
impl Deref for Blob {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// a table that allows you to tlb[col_type](extract) without matching

pub trait BindTo {
    fn bind_to(self, col: i32, stmt: *mut sqlite3_stmt) -> i32;
}

// impl bind for types that are supported

impl BindTo for self::Integer {
    fn bind_to(self, col: i32, stmt: *mut sqlite3_stmt) -> i32 {
        unsafe { sqlite3_bind_int64(stmt, col, self) }
    }
}
impl BindTo for self::Float {
    fn bind_to(self, col: i32, stmt: *mut sqlite3_stmt) -> i32 {
        unsafe { sqlite3_bind_double(stmt, col, self) }
    }
}
impl BindTo for &str {
    fn bind_to(self, col: i32, stmt: *mut sqlite3_stmt) -> i32 {
        unsafe {
            let transient: sqlite3_destructor_type = std::mem::transmute(-1_isize);
            sqlite3_bind_text(
                stmt,
                col,
                self.as_ptr().cast(),
                self.len() as i32,
                transient,
            )
        }
    }
}
impl BindTo for String {
    fn bind_to(self, col: i32, stmt: *mut sqlite3_stmt) -> i32 {
        self.as_str().bind_to(col, stmt)
    }
}

pub trait ExtractFrom {
    fn extract_from(stmt: *mut sqlite3_stmt, col: i32) -> Self;
}
impl ExtractFrom for Integer {
    fn extract_from(stmt: *mut sqlite3_stmt, col: i32) -> self::Integer {
        unsafe { sqlite3_column_int64(stmt, col) }
    }
}
impl ExtractFrom for Float {
    fn extract_from(stmt: *mut sqlite3_stmt, col: i32) -> Self {
        unsafe { sqlite3_column_double(stmt, col) }
    }
}

// #[repr(transparent)]
pub struct Statement<'conn> {
    ptr: *mut sqlite3_stmt,
    expected_bind_param_count: i32,
    sql: CString,
    // conn: Arc<ConnectionHandle>,
    conn: &'conn ConnectionHandle,
}
#[derive(PartialEq)]
pub enum StepResult {
    Row,
    Interrupted,
    Done,
    Error(ExtendedDatabaseErrorInfo)
}

impl<'conn> Statement<'conn> {
    pub(crate) fn prepare(connection: &'conn ConnectionHandle, sql: &str) -> Result<Self, String> {
        let sql = std::ffi::CString::new(sql).expect("Non null terminated string");
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sqlite3_prepare_v2(
                **connection,
                sql.as_ptr(),
                sql.as_bytes_with_nul().len() as i32,
                &mut ptr,
                std::ptr::null_mut(),
            );
        }
        if ptr.is_null() {
            return Err("BOOP BOOP BOP BOP **KABOOM** Nullptr".to_string());
        }
        let max = unsafe { libsqlite3_sys::sqlite3_bind_parameter_count(ptr) };
        let s = Self {
            ptr,
            expected_bind_param_count: max,
            conn: connection,
            sql,
        };
        Ok(s)
    }
    pub(crate) fn rollback(&mut self) {
        if unsafe { libsqlite3_sys::sqlite3_get_autocommit(**self.conn) } != 0 {
            return; // Already in autocommit mode, nothing to do
        }

        if unsafe { libsqlite3_sys::sqlite3_get_autocommit(**self.conn) } == 0 {
            // 2. Clear the poisoned state
            let _ = unsafe {
                libsqlite3_sys::sqlite3_exec(
                    **self.conn,
                    c"ROLLBACK;".as_ptr().cast(),
                    None,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            };
        }
    }
    /// attempts to step the stmt, waits if the disk or sqlite is busy
    pub(crate) fn step(&mut self) -> StepResult {
        debug_assert!(self.ptr.is_null() == false);
        // let conn = self.conn.get_mut_for_stmt(self);


        // check the autocommit state
        self.conn.get_autocommit();

        let mut result = unsafe { sqlite3_step(self.ptr) };


        
        while result == SQLITE_BUSY {
            result = unsafe { sqlite3_step(self.ptr) };
            for _ in 0.100 {
                core::hint::spin_loop();
            }
            std::thread::yield_now();
        }
        match result {
            SQLITE_DONE => StepResult::Done,
            SQLITE_ROW => StepResult::Row,
            SQLITE_INTERRUPT => {
                StepResult::Interrupted
            }
            _=> {
                StepResult::Error(
                    self.conn.last_error()
                )
            }
        }
    }
    pub(crate) fn bind_params_count(&self) -> i32 {
        self.expected_bind_param_count
    }
    pub(crate) fn reset(&mut self) {
        debug_assert!(self.ptr.is_null() == false);
        let _rc = unsafe { sqlite3_reset(self.ptr) };
    }
    pub(crate) fn column_count(&self) -> i32 {
        unsafe { sqlite3_column_count(self.ptr) }
    }
    pub(crate) fn column_type(&self, c: i32) -> i32 {
        unsafe { sqlite3_column_type(self.ptr, c) }
    }
    // pub (crate) fn bind_vtbl(&self) -> _ {
    //     // let t = [self.bind::<Integer>,self.bind::<Float>,self.bind::<String>];
    // }
    pub(crate) fn finalize(self) {}
    pub(crate) fn text(&mut self, col: i32) -> String {
        unsafe { super::utils::stmt_text_to_string_lossy_empty_if_null(self.ptr, col) }
    }
    pub(crate) fn bind<T>(self, col: i32, val: T) -> Self
    where
        T: BindTo,
    {
        #[cfg(debug_assertions)]
        {
            if col < 1 || col > self.expected_bind_param_count {
                panic!(
                    "Bind Error: Column index {} is out of range (1..{})",
                    col, self.expected_bind_param_count
                );
            }
        }
        let rc = val.bind_to(col, self.ptr);
        debug_assert_eq!(
            rc,
            libsqlite3_sys::SQLITE_OK,
            "SQLite Bind failed with error code: {}",
            rc
        );
        self
    }
    pub(crate) fn extract<T>(&self, col: i32) -> T
    where
        T: ExtractFrom,
    {
        let t = T::extract_from(self.ptr, col);
        t
    }
    // pub(crate) fn column_count(&self) -> i32 {
    //     unsafe {
    //         sqlite3_column_count(self.0)
    //     }
    // }
}
// not sure if needed but DONT USE WITH MORE THAN 1 THREAD AT SAME TIME. DO NOT CLONE
unsafe impl<'conn> Send for Statement<'conn> {}

impl<'conn> std::ops::Deref for Statement<'conn> {
    type Target = *mut sqlite3_stmt;
    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

// impl std::ops::DerefMut for Statement {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.ptr
//     }
// }

impl<'conn> Drop for Statement<'conn> {
    fn drop(&mut self) {
        let _ = unsafe { sqlite3_finalize(self.ptr) };
    }
}
