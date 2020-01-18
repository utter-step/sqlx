use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::Sqlite;
use crate::sqlite::value::SqliteValue;
use crate::types::HasSqlType;

impl HasSqlType<i8> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Int,
        }
    }
}

impl HasSqlType<i16> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Int,
        }
    }
}

impl HasSqlType<i32> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Int,
        }
    }
}

impl Encode<Sqlite> for i32 {
    fn encode(&self, values: &mut Vec<SqliteValue>) {
        values.push(SqliteValue::Int((*self).into()));
    }
}

impl HasSqlType<i64> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Int,
        }
    }
}
