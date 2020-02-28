use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

use futures_core::future::BoxFuture;
use futures_util::TryFutureExt;

use crate::database::Database;
use crate::describe::Describe;
use crate::executor::Executor;
use crate::pool::{Pool, PoolConnection};
use crate::url::Url;

/// Represents a single database connection rather than a pool of database connections.
///
/// Prefer running queries from [Pool] unless there is a specific need for a single, continuous
/// connection.
pub trait Connection
where
    Self: Send + 'static,
{
    type Database: Database;

    /// Close this database connection.
    fn close(self) -> BoxFuture<'static, crate::Result<()>>;

    /// Verifies a connection to the database is still alive.
    fn ping(&mut self) -> BoxFuture<crate::Result<()>>;

    #[doc(hidden)]
    fn describe<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>>;
}

/// Represents a type that can directly establish a new connection.
pub trait Connect: Connection {
    /// Establish a new database connection.
    fn connect<T>(url: T) -> BoxFuture<'static, crate::Result<Self>>
    where
        T: TryInto<Url, Error = crate::Error>,
        Self: Sized;
}
