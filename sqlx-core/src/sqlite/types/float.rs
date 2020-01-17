use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::sqlite::types::{SqliteTypeInfo, ValueKind};
use crate::sqlite::Sqlite;
use crate::types::HasSqlType;

impl HasSqlType<f32> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Double,
        }
    }
}

impl HasSqlType<f64> for Sqlite {
    fn type_info() -> SqliteTypeInfo {
        SqliteTypeInfo {
            kind: ValueKind::Double,
        }
    }
}
