use std::future::Future;
use std::marker::PhantomData;

use futures_util::future::ready;
use futures_util::TryFutureExt;

use crate::arguments::Arguments;
use crate::arguments::IntoArguments;
use crate::cursor::Cursor;
use crate::database::{Database, HasCursor, HasRow};
use crate::encode::Encode;
use crate::executor::{Execute, Executor};
use crate::types::Type;

/// Dynamic SQL query with bind parameters. Returned by [query].
///
/// The methods on this struct should be passed a reference to [crate::Pool] or one of
/// the connection types.
pub struct Query<'q, DB, T = <DB as Database>::Arguments>
where
    DB: Database,
{
    query: &'q str,
    arguments: T,
    database: PhantomData<DB>,
}

impl<'q, DB, P> Execute<'q, DB> for Query<'q, DB, P>
where
    DB: Database,
    P: IntoArguments<DB> + Send,
{
    fn into_parts(self) -> (&'q str, Option<<DB as Database>::Arguments>) {
        (self.query, Some(self.arguments.into_arguments()))
    }
}

impl<'q, DB, P> Query<'q, DB, P>
where
    DB: Database,
    P: IntoArguments<DB> + Send,
{
    pub fn execute<'e, E>(self, executor: E) -> impl Future<Output = crate::Result<u64>> + 'e
    where
        E: Executor<'e, Database = DB>,
        'q: 'e,
    {
        executor.execute(self)
    }

    pub fn fetch<'e, E>(self, executor: E) -> <DB as HasCursor<'e>>::Cursor
    where
        E: Executor<'e, Database = DB>,
        'q: 'e,
    {
        executor.execute(self)
    }

    pub fn fetch_optional<'e, E>(
        self,
        executor: E,
    ) -> impl Future<Output = crate::Result<Option<<DB as HasRow<'e>>::Row>>>
    where
        E: Executor<'e, Database = DB>,
        'q: 'e,
    {
        executor.execute(self).first()
    }

    pub fn fetch_one<'e, E>(
        self,
        executor: E,
    ) -> impl Future<Output = crate::Result<<DB as HasRow<'e>>::Row>>
    where
        E: Executor<'e, Database = DB>,
        'q: 'e,
    {
        self.fetch_optional(executor).and_then(|row| match row {
            Some(row) => ready(Ok(row)),
            None => ready(Err(crate::Error::NotFound)),
        })
    }
}

impl<'q, DB> Query<'q, DB>
where
    DB: Database,
{
    /// Bind a value for use with this SQL query.
    ///
    /// # Logic Safety
    ///
    /// This function should be used with care, as SQLx cannot validate
    /// that the value is of the right type nor can it validate that you have
    /// passed the correct number of parameters.
    pub fn bind<T>(mut self, value: T) -> Self
    where
        T: Type<DB>,
        T: Encode<DB>,
    {
        self.arguments.add(value);
        self
    }
}

/// Construct a full SQL query that can be chained to bind parameters and executed.
///
/// # Examples
///
/// ```ignore
/// let names: Vec<String> = sqlx::query("SELECT name FROM users WHERE active = ?")
///     .bind(false) // [active = ?]
///     .fetch(&mut connection) // -> Stream<Item = impl Row>
///     .map_ok(|row| row.name("name")) // -> Stream<Item = String>
///     .try_collect().await?; // -> Vec<String>
/// ```
pub fn query<DB>(sql: &str) -> Query<DB>
where
    DB: Database,
{
    Query {
        database: PhantomData,
        arguments: Default::default(),
        query: sql,
    }
}
