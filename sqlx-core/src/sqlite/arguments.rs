use crate::arguments::Arguments;
use crate::encode::Encode;
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

pub enum Value {
    // TODO: Take by reference to remove the allocation
    Text(String),

    // TODO: Take by reference to remove the allocation
    Blob(Vec<u8>),

    Double(f64),

    Int(i64),

    Null,
}

#[derive(Default)]
pub struct SqliteArguments {
    values: Vec<Value>,
}

impl Arguments for SqliteArguments {
    type Database = Sqlite;

    fn len(&self) -> usize {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn reserve(&mut self, len: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        Self::Database: HasSqlType<T>,
        T: Encode<Self::Database>,
    {
        todo!()
    }
}
