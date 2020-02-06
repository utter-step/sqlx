//! Contains the Row and FromRow traits.

use crate::database::Database;
use crate::decode::Decode;
use crate::types::Type;

// TODO: Bring back [RowIndex]

//pub trait RowIndex<R: ?Sized>
//where
//    R: Row,
//{
//    fn try_get<T>(&self, row: &R) -> crate::Result<T>
//    where
//        R::Database: HasSqlType<T>,
//        T: Decode<R::Database>;
//}

/// A single row of a result set returned from the database.
pub trait Row<'c>: Unpin + Send {
    type Database: Database;

    /// Returns the number of columns in the row.
    fn len(&self) -> usize;

    fn try_get<'de: 'c, T>(&'de self, index: usize) -> crate::Result<T>
    where
        T: Type<Self::Database>,
        T: Decode<'de, Self::Database>;
}

/// A **record** that can be built from a [`Row`] returned from by the database.
pub trait FromRow<'c, R>
where
    Self: Sized,
    R: Row<'c>,
{
    fn from_row(row: R) -> crate::Result<Self>;
}

// TODO: Implement [FromRow] for tuples up to x8
