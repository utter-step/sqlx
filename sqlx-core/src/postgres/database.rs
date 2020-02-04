use crate::database::{Database, HasCursor, HasRawValue, HasRow};

/// **Postgres** database driver.
pub struct Postgres;

impl Database for Postgres {
    type Connection = super::PgConnection;

    type Arguments = super::PgArguments;

    type TypeInfo = super::PgTypeInfo;

    type TableId = u32;

    type RawBuffer = Vec<u8>;
}

impl<'c> HasRawValue<'c> for Postgres {
    type RawValue = Option<&'c [u8]>;
}

impl<'c> HasRow<'c> for Postgres {
    type Database = Postgres;

    type Row = super::PgRow<'c>;
}

impl<'c> HasCursor<'c> for Postgres {
    type Database = Postgres;

    type Cursor = super::PgCursor<'c>;
}
