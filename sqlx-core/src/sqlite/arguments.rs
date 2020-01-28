use crate::arguments::Arguments;
use crate::encode::Encode;
use crate::sqlite::value::SqliteValue;
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

#[derive(Default)]
pub struct SqliteArguments {
    values: Vec<SqliteValue>,
}

impl Arguments for SqliteArguments {
    type Database = Sqlite;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn reserve(&mut self, len: usize, _size_hint: usize) {
        self.values.reserve(1);
    }

    fn add<T>(&mut self, value: T)
    where
        Self::Database: HasSqlType<T>,
        T: Encode<Self::Database>,
    {
        value.encode(&mut self.values);
    }
}
