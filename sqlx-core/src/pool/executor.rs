use std::ops::DerefMut;

use futures_core::{future::BoxFuture, stream::BoxStream};
use futures_util::StreamExt;

use crate::{
    connection::{Connect, Connection},
    describe::Describe,
    executor::Executor,
    pool::Pool,
    Database,
};

use super::PoolConnection;
use crate::database::HasCursor;
use crate::executor::Execute;

impl<'p, C> Executor<'p> for &'p Pool<C>
where
    C: Connection + Connect<Connection = C>,
    for<'con> &'con mut C: Executor<'con>,
{
    type Database = C::Database;

    fn execute<'q, E>(self, query: E) -> <Self::Database as HasCursor<'p>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn execute_by_ref<'q, 'e, E>(
        &'e mut self,
        query: E,
    ) -> <Self::Database as HasCursor<'_>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }
}

impl<'c, C> Executor<'c> for &'c mut PoolConnection<C>
where
    C: Connection + Connect<Connection = C>,
    for<'con> &'con mut C: Executor<'con>,
{
    type Database = C::Database;

    fn execute<'q, E>(self, query: E) -> <Self::Database as HasCursor<'c>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }

    fn execute_by_ref<'q, 'e, E>(
        &'e mut self,
        query: E,
    ) -> <Self::Database as HasCursor<'_>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        todo!()
    }
}

//
//impl<C> Executor for PoolConnection<C>
//where
//    C: Connection + Connect<Connection = C>,
//{
//    type Database = <C as Executor>::Database;
//
//    fn send<'e, 'q: 'e>(&'e mut self, commands: &'q str) -> BoxFuture<'e, crate::Result<()>> {
//        self.deref_mut().send(commands)
//    }
//
//    fn execute<'e, 'q: 'e>(
//        &'e mut self,
//        query: &'q str,
//        args: <<C as Executor>::Database as Database>::Arguments,
//    ) -> BoxFuture<'e, crate::Result<u64>> {
//        self.deref_mut().execute(query, args)
//    }
//
//    fn fetch<'e, 'q: 'e>(
//        &'e mut self,
//        query: &'q str,
//        args: <<C as Executor>::Database as Database>::Arguments,
//    ) -> BoxStream<'e, crate::Result<<<C as Executor>::Database as Database>::Row>> {
//        self.deref_mut().fetch(query, args)
//    }
//
//    fn fetch_optional<'e, 'q: 'e>(
//        &'e mut self,
//        query: &'q str,
//        args: <<C as Executor>::Database as Database>::Arguments,
//    ) -> BoxFuture<'e, crate::Result<Option<<<C as Executor>::Database as Database>::Row>>> {
//        self.deref_mut().fetch_optional(query, args)
//    }
//
//    fn describe<'e, 'q: 'e>(
//        &'e mut self,
//        query: &'q str,
//    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>> {
//        self.deref_mut().describe(query)
//    }
//}
