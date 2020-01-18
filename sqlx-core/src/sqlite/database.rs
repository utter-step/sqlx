use crate::database::Database;
use crate::sqlite::value::SqliteValue;

/// **SQLite** database driver.
pub struct Sqlite;

impl Database for Sqlite {
    type Connection = super::SqliteConnection;

    type Arguments = super::SqliteArguments;

    type Row = super::SqliteRow;

    type TypeInfo = super::SqliteTypeInfo;

    // TODO: Not sure what this should be
    type TableId = u32;

    type Buffer = Vec<SqliteValue>;

    type Value = SqliteValue;
}
