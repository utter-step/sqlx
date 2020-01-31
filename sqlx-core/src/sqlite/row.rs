use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use libsqlite3_sys::sqlite3_column_int;

use crate::database::{HasRawRow, HasRawValue};
use crate::decode::Decode;
use crate::row::RawRow;
use crate::sqlite::arguments::SqliteValue;
use crate::sqlite::statement::Statement;
use crate::sqlite::Sqlite;
use crate::types::Type;

pub struct SqliteValueProxy<'a> {
    row: &'a SqliteRow<'a>,
    index: i32,
}

pub struct SqliteRow<'a> {
    // pub(super) conn: &'a super::SqliteConnection,
    pub(super) statement: &'a Statement,
    // pub(super) phantom: PhantomData<&'a ()>,
    // pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

#[allow(unsafe_code)]
unsafe impl<'a> Sync for SqliteRow<'a> {}

impl<'a> RawRow<'a> for SqliteRow<'a> {
    type Database = Sqlite;

    fn len(&self) -> usize {
        // self.values.len()
        todo!()
    }

    fn get<T>(&'a self, index: usize) -> T
    where
        T: Type<Self::Database>,
        T: Decode<'a, Self::Database>,
    {
        // FIXME: Handle UNWRAP better
        Decode::decode(SqliteValueProxy {
            row: self,
            index: index as i32,
        })
        .unwrap()
    }
}

impl<'a> SqliteValueProxy<'a> {
    pub(super) fn int(&self) -> i32 {
        // <https://www.sqlite.org/c3ref/column_int.html>
        #[allow(unsafe_code)]
        unsafe {
            sqlite3_column_int(self.row.statement.0.as_ptr(), self.index)
        }
    }
}

impl<'a> HasRawValue<'a> for Sqlite {
    type RawValue = SqliteValueProxy<'a>;
}

impl<'a> HasRawRow<'a> for Sqlite {
    type Database = Sqlite;

    type RawRow = SqliteRow<'a>;
}
