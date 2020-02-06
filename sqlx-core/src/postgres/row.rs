use std::collections::HashMap;
use std::sync::Arc;

use crate::decode::Decode;
use crate::postgres::protocol::DataRow;
use crate::postgres::{cursor::Source, PgCursor, Postgres};
use crate::row::Row;
use crate::types::Type;

// 's: the lifetime of the database server connection or socket
pub struct PgRow<'s> {
    pub(super) source: Option<Source<'s>>,
    pub(super) data: Option<DataRow<'s>>,
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl<'s> Row<'s> for PgRow<'s> {
    type Database = Postgres;

    fn len(&self) -> usize {
        // self.data.len()
        todo!()
    }

    fn try_get<'de: 's, T>(&'de self, index: usize) -> crate::Result<T>
    where
        T: Type<Self::Database>,
        T: Decode<'de, Self::Database>,
    {
        todo!()
    }
}
