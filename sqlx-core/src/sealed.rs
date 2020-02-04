use crate::connection::{Connect, Connection};
use crate::executor::Executor;
use crate::pool::{Pool, PoolConnection};
use crate::transaction::Transaction;

// Public and un-exported trait to be used to seal fully public traits where
// the interface on the trait should be considered unstable and not-for-external-use

pub trait Sealed {}

// Add additional impls here as necessary
// The logic is the concrete types below can have sealed traits implemented for them

impl<T> Sealed for &'_ mut Transaction<T>
where
    T: Connection,
    T: Executor<'static>,
{
}

impl<C> Sealed for &'_ Pool<C> {}

impl<C> Sealed for &'_ mut PoolConnection<C>
where
    C: Connection,
    C: Connect<Connection = C>,
{
}

#[cfg(feature = "postgres")]
mod postgres {
    use crate::postgres::PgConnection;

    impl super::Sealed for &'_ mut PgConnection {}
}
