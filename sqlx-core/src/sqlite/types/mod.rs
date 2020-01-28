use std::fmt::{self, Display};

use crate::types::TypeInfo;

mod bytes;
mod float;
mod int;
mod str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueKind {
    // `Null` is used when we have no idea what the type is
    // SQLite does not infer bind type params or result columns
    // when the result is an expression
    Null,

    Text,
    Blob,
    Int,
    Double,
}

#[derive(Debug, Clone, Copy)]
pub struct SqliteTypeInfo {
    kind: ValueKind,
}

impl SqliteTypeInfo {
    pub(crate) const NULL: SqliteTypeInfo = SqliteTypeInfo {
        kind: ValueKind::Null,
    };

    pub(crate) fn new(kind: ValueKind) -> Self {
        SqliteTypeInfo { kind }
    }

    /// Returns `true` if the type could not be resolved.
    ///
    ///  * Bind parameters will have a `NULL` type.
    ///
    ///  * Result columns that are expressions will have a `NULL` type.
    ///
    pub fn is_null(&self) -> bool {
        self.kind == ValueKind::Null
    }
}

impl TypeInfo for SqliteTypeInfo {
    fn compatible(&self, other: &Self) -> bool {
        self.kind == self.kind
    }
}

impl Display for SqliteTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}
