use std::fmt::Display;

use crate::arguments::Arguments;
use crate::connection::Connection;
use crate::cursor::Cursor;
use crate::row::Row;
use crate::types::TypeInfo;

/// A database driver.
///
/// This trait encapsulates a complete driver implementation to a specific
/// database (e.g., MySQL, Postgres).
pub trait Database
where
    Self: Sized + 'static,
    Self: for<'c> HasRawValue<'c>,
    Self: for<'c> HasRow<'c, Database = Self>,
    Self: for<'con> HasCursor<'con, Database = Self>,
{
    /// The concrete `Connection` implementation for this database.
    type Connection: Connection<Database = Self>;

    /// The concrete `Arguments` implementation for this database.
    type Arguments: Arguments<Database = Self>;

    /// The concrete `TypeInfo` implementation for this database.
    type TypeInfo: TypeInfo;

    /// The Rust type of table identifiers for this database.
    type TableId: Display + Clone;

    type RawBuffer;
}

// 's: the lifetime of the database server connection or socket
pub trait HasRawValue<'s> {
    type RawValue;
}

// 's: the lifetime of the database server connection or socket
pub trait HasRow<'s> {
    type Database: Database;

    type Row: Row<'s, Database = Self::Database>;
}

// 'e: the lifetime of the Executor reference
pub trait HasCursor<'e> {
    type Database: Database;

    type Cursor: Cursor<'e, Database = Self::Database>;
}
