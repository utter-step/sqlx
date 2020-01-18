use core::ptr::null_mut;
use core::i32;

use std::convert::TryInto;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use libsqlite3_sys::{sqlite3_prepare_v3, SQLITE_OK, sqlite3_stmt, SQLITE_PREPARE_NO_VTAB, SQLITE_PREPARE_PERSISTENT};

use crate::describe::Describe;
use crate::executor::Executor;
use crate::sqlite::{Sqlite, SqliteArguments, SqliteConnection, SqliteRow};

impl SqliteConnection {
    fn prepare(&mut self, query: &str) {
        // TODO: Cache?

        let mut statement: *mut sqlite3_stmt = null_mut();

        // TODO: Handle the error when there are internal NULs in the query
        let mut sql = query.as_bytes();
        
        let sql_len = query.len();
        if sql_len > (i32::MAX as usize) {
            panic!("query too large");
        }

        // TODO: Contribute this back to libsqlite3-sys, these flags should be u32
        let flags = (SQLITE_PREPARE_PERSISTENT | SQLITE_PREPARE_NO_VTAB) as u32; 

        // SAFE: [filename] and [statement] are valid
        // <https://www.sqlite.org/c3ref/prepare.html>
        #[allow(unsafe_code)]
        let status = unsafe {
            sqlite3_prepare_v3(self.handle.as_ptr(), sql.as_ptr() as *const i8, sql.len() as i32, flags, &mut statement, null_mut())
        };

        if status != SQLITE_OK {
            // TODO: Add handling of sqlite errors
            // We need to bubble up as a [DatabaseError]
            panic!("not ok: {}", status);
        }

        todo!()
    }
}

impl Executor for SqliteConnection {
    type Database = Sqlite;

    fn send<'e, 'q: 'e>(&'e mut self, query: &'q str) -> BoxFuture<'e, crate::Result<()>> {
        todo!()
    }

    fn execute<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: SqliteArguments,
    ) -> BoxFuture<'e, crate::Result<u64>> {
        let statement = self.prepare(query);
        todo!()
    }

    fn fetch<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: SqliteArguments,
    ) -> BoxStream<'e, crate::Result<SqliteRow>> {
        let statement = self.prepare(query);
        todo!()
    }

    fn describe<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>> {
        let statement = self.prepare(query);
        todo!()
    }
}
