use std::collections::HashMap;
use std::sync::Arc;

use crate::decode::Decode;
use crate::row::{Row, RowIndex};
use crate::sqlite::value::SqliteValue;
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

pub struct SqliteRow {
    // pub(super) statement: Statement<'cur>,
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
