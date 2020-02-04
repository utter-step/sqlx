use core::fmt::{self, Debug, Display};

use crate::postgres::protocol::TypeId;
use crate::types::TypeInfo;

//mod bool;
//mod bytes;
//mod float;
mod int;
mod str;
//
//#[cfg(feature = "chrono")]
//mod chrono;
//
//#[cfg(feature = "uuid")]
//mod uuid;

#[derive(Debug, Clone)]
pub struct PgTypeInfo {
    pub(crate) id: TypeId,
}

impl PgTypeInfo {
    pub(crate) fn new(id: TypeId) -> Self {
        Self { id }
    }

    /// Create a `PgTypeInfo` from a type's object identifier.
    ///
    /// The object identifier (OID) of a type can be queried with:
    ///
    /// ```text
    /// SELECT oid FROM pg_type WHERE typname = <name>
    /// ```
    pub fn with_oid(oid: u32) -> Self {
        Self { id: TypeId(oid) }
    }
}

impl Display for PgTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.0)
    }
}

impl TypeInfo for PgTypeInfo {
    fn compatible(&self, other: &Self) -> bool {
        // TODO: Support "Text-like" types
        self.id.0 == other.id.0
    }
}
