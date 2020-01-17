use crate::decode::Decode;
use crate::row::{Row, RowIndex};
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

pub struct SqliteRow {
    // TODO
}

impl Row for SqliteRow {
    type Database = Sqlite;

    fn len(&self) -> usize {
        todo!()
    }

    fn get<T, I>(&self, index: I) -> T
    where
        Self::Database: HasSqlType<T>,
        I: RowIndex<Self>,
        T: Decode<Self::Database>,
    {
        todo!()
    }
}
