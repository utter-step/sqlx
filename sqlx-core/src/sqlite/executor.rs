use core::i32;
use core::ptr::{null_mut, NonNull};

use std::convert::TryInto;
use std::sync::Arc;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use libsqlite3_sys::{
    sqlite3_prepare_v3, sqlite3_stmt, SQLITE_OK, SQLITE_PREPARE_NO_VTAB, SQLITE_PREPARE_PERSISTENT,
};

use crate::describe::Describe;
use crate::executor::Executor;
use crate::sqlite::statement::{Statement, Step};
use crate::sqlite::{Sqlite, SqliteArguments, SqliteCursor, SqliteConnection, SqliteRow, SqliteRow2};

impl SqliteConnection {
    fn prepare(&mut self, query: &str) -> crate::Result<Statement> {
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

        // <https://www.sqlite.org/c3ref/prepare.html>
        #[allow(unsafe_code)]
        let status = unsafe {
            sqlite3_prepare_v3(
                self.handle.as_ptr(),
                sql.as_ptr() as *const i8,
                sql.len() as i32,
                flags,
                &mut statement,
                null_mut(),
            )
        };

        if status != SQLITE_OK {
            // TODO: Add handling of sqlite errors
            // We need to bubble up as a [DatabaseError]
            panic!("not ok: {}", status);
        }

        // TODO: Handle the error when NULL is returned
        Ok(Statement(NonNull::new(statement).unwrap()))
    }
}

// Executor2
impl SqliteConnection {
    fn fetch2<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: SqliteArguments,
    ) -> SqliteCursor<'e> {
        let mut statement = self.prepare(query);
        let columns = statement.as_mut().map(|mut s| self.column_names(&mut s)).ok();

        SqliteCursor {
            statement,
            columns,
            connection: self
        }

        // Box::pin(async_stream::try_stream! {
        //     let mut statement = self.prepare(query)?;

        //     let columns = self.column_names(&mut statement)?;

        //     while let Step::Row = statement.step()? {
        //         let mut values = Vec::with_capacity(columns.len());

        //         for i in 0..columns.len() {
        //             values.push(statement.value(i));
        //         }

        //         yield SqliteRow2 {
        //             // values: values.into_boxed_slice(),
        //             // columns: Arc::clone(&columns),
        //             statement: statement.clone()
        //         }
        //     }
        // })
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
        let mut statement = self.prepare(query).unwrap();

        statement.step().unwrap();
        // statement.reset();

        todo!();
    }

    fn fetch<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: SqliteArguments,
    ) -> BoxStream<'e, crate::Result<SqliteRow>> {
        Box::pin(async_stream::try_stream! {
            let mut statement = self.prepare(query)?;

            let columns = self.column_names(&mut statement);

            while let Step::Row = statement.step()? {
                let mut values = Vec::with_capacity(columns.len());

                for i in 0..columns.len() {
                    values.push(statement.value(i));
                }

                yield SqliteRow {
                    values: values.into_boxed_slice(),
                    columns: Arc::clone(&columns),
                }
            }
        })
    }

    fn describe<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>> {
        Box::pin(async move { self.prepare(query)?.describe() })
    }
}
