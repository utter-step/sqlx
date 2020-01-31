use crate::database::{Database, HasRawRow, HasRawValue};
use crate::sqlite::arguments::{SqliteArguments, SqliteValue};
use crate::sqlite::connection::SqliteConnection;
use crate::sqlite::row::{SqliteRow, SqliteValueProxy};
use crate::sqlite::types::SqliteTypeInfo;

/// **SQLite** database driver.
pub struct Sqlite;

impl Database for Sqlite {
    type Connection = SqliteConnection;

    type Arguments = SqliteArguments;

    // type Row = SqliteRow<'static>;

    type TypeInfo = SqliteTypeInfo;

    // TODO: Not sure what this should be
    type TableId = u32;

    type RawBuffer = Vec<SqliteValue>;

    // type Value<'c> = SqliteValueProxy<'c>;
}
