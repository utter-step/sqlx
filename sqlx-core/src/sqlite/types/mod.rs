use std::fmt::{self, Display};

use crate::types::TypeInfo;

mod bytes;
mod float;
mod int;
mod str;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Text,
    Blob,
    Int,
    Double,
}

#[derive(Debug, Clone)]
pub struct SqliteTypeInfo {
    kind: ValueKind,
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
