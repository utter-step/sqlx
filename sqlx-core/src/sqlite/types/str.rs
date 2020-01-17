use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

impl HasSqlType<str> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Text,
        }
    }
}

impl HasSqlType<String> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        <Self as HasSqlType<str>>::type_info()
    }
}
