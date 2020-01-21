use std::sync::Arc;
use std::collections::HashMap;

use crate::decode::Decode;
use crate::row::{Row, RowIndex};
use crate::sqlite::Sqlite;
use crate::sqlite::value::SqliteValue;
use crate::types::HasSqlType;

pub struct SqliteRow {
    // TODO: Switch to "checking out" a statement ptr so we can use this directly to decode values
    //       from. Decoding to a [SqliteValue] isn't "good enough (tm)"

    // pub(super) statement: Statement,

    pub(super) values: Box<[Option<SqliteValue>]>,
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl Row for SqliteRow {
    type Database = Sqlite;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn get<T, I>(&self, index: I) -> T
    where
        Self::Database: HasSqlType<T>,
        I: RowIndex<Self>,
        T: Decode<Self::Database>,
    {
        index.try_get(self).unwrap()
    }
}

// More of the same.. really need to fix https://github.com/launchbadge/sqlx/issues/49

impl RowIndex<SqliteRow> for usize {
    fn try_get<T>(&self, row: &SqliteRow) -> crate::Result<T>
    where
        <SqliteRow as Row>::Database: HasSqlType<T>,
        T: Decode<<SqliteRow as Row>::Database>,
    {
        Ok(Decode::decode_nullable(row.values[*self].clone())?)
    }
}

impl RowIndex<SqliteRow> for &'_ str {
    fn try_get<T>(&self, row: &SqliteRow) -> crate::Result<T>
    where
        <SqliteRow as Row>::Database: HasSqlType<T>,
        T: Decode<<SqliteRow as Row>::Database>,
    {
        let index = row
            .columns
            .get(*self)
            .ok_or_else(|| crate::Error::ColumnNotFound((*self).into()))?;

        let value = Decode::decode_nullable(row.values[*index].clone())?;

        Ok(value)
    }
}

impl_from_row_for_row!(SqliteRow);
