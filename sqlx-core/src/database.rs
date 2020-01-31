use std::fmt::Display;

use crate::arguments::Arguments;
use crate::connection::Connection;
use crate::cursor::Cursor;
use crate::row::RawRow;
use crate::types::TypeInfo;

/// A database driver.
///
/// This trait encapsulates a complete driver implementation to a specific
/// database (e.g., MySQL, Postgres).
pub trait Database
where
    Self: Sized,
    Self: for<'a> HasRawRow<'a, Database = Self>,
    Self: for<'a> HasCursor<'a, Database = Self>,
    Self: for<'a> HasRawValue<'a>,
{
    /// The concrete `Connection` implementation for this database.
    type Connection: Connection<Database = Self>;

    /// The concrete `Arguments` implementation for this database.
    type Arguments: Arguments<Database = Self>;

    /// The concrete `TypeInfo` implementation for this database.
    type TypeInfo: TypeInfo;

    /// The Rust type of table identifiers for this database.
    type TableId: Display + Clone;

    // TODO: Add docs
    type RawBuffer;
}

// TODO: Add docs
pub trait HasRawValue<'a> {
    type RawValue;
}

// TODO: Add docs
pub trait HasRawRow<'a> {
    type Database: Database;

    type RawRow: RawRow<'a, Database = Self::Database>;
}

// TODO: Add docs
pub trait HasCursor<'a> {
    type Database: Database;

    type Cursor: Cursor<'a, Database = Self::Database>;
}
