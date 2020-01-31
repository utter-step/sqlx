use futures_core::future::BoxFuture;

use crate::database::{Database, HasRawRow};

// TODO: Document this
pub trait Cursor<'con> {
    type Database: Database;

    // Corresponds to [`futures::Stream::try_next`]
    fn try_next<'cur>(
        &'cur mut self,
    ) -> BoxFuture<'cur, crate::Result<Option<<Self::Database as HasRawRow<'cur>>::RawRow>>>;
}
