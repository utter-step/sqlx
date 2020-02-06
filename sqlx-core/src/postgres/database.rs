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

// 's: the lifetime of the database server connection or socket
impl<'s> HasRow<'s> for Postgres {
    // TODO: Can we drop the `type Database = _`
    type Database = Postgres;

    type Row = super::PgRow<'s>;
}

// 'e: the lifetime of the Executor reference
impl<'e> HasCursor<'e> for Postgres {
    // TODO: Can we drop the `type Database = _`
    type Database = Postgres;

    type Cursor = super::PgCursor<'e>;
}

impl<'s> HasRawValue<'s> for Postgres {
    type RawValue = Option<&'s [u8]>;
}
