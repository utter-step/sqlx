use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::value::SqliteValue;
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

impl HasSqlType<i8> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl HasSqlType<i16> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl HasSqlType<i32> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}

impl Encode<Sqlite> for i32 {
    fn encode(&self, values: &mut Vec<SqliteValue>) {
        values.push(SqliteValue::Int((*self).into()));
    }
}

impl Decode<Sqlite> for i32 {
    fn decode(value: SqliteValue) -> Result<i32, DecodeError> {
        Ok(match value {
            // TODO: Cast?
            SqliteValue::Int(val) => val as i32,

            _ => unimplemented!(),
        })
    }
}

impl HasSqlType<i64> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Int)
    }
}
