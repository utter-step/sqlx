use std::convert::TryInto;

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use crate::describe::Describe;
use crate::executor::Executor;
use crate::sqlite::{Sqlite, SqliteArguments, SqliteConnection, SqliteRow};

impl SqliteConnection {
    fn prepare(&mut self) {
        // <https://www.sqlite.org/c3ref/prepare.html>
        // 


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
        todo!()
    }

    fn fetch<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: SqliteArguments,
    ) -> BoxStream<'e, crate::Result<SqliteRow>> {
        todo!()
    }

    fn describe<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>> {
        todo!()
    }
}
