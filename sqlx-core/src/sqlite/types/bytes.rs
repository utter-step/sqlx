use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

impl HasSqlType<[u8]> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo::new(ValueKind::Blob)
    }
}

impl HasSqlType<Vec<u8>> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        <Self as HasSqlType<[u8]>>::type_info()
    }
}
