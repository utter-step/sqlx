use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::arguments::SqliteValue;
use crate::sqlite::row::SqliteValueProxy;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::Sqlite;
use crate::types::Type;

impl Type<Sqlite> for i8 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl Type<Sqlite> for i16 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl Type<Sqlite> for i32 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl Encode<Sqlite> for i32 {
    fn encode(&self, values: &mut Vec<SqliteValue>) {
        values.push(SqliteValue::Int((*self).into()));
    }
}

impl<'a> Decode<'a, Sqlite> for i32 {
    fn decode(value: SqliteValueProxy<'a>) -> Result<i32, DecodeError> {
        // Even NULL will come through as 0
        Ok(value.int())
    }
}

impl Type<Sqlite> for i64 {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}
