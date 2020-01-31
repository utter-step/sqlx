//! Contains the Row and FromRow traits.

use crate::database::{Database, HasRawValue};
use crate::decode::Decode;
use crate::types::Type;

pub trait RawRow<'a>: Unpin + Send {
    type Database: Database;

    /// Returns `true` if the row contains no values.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of values in the row.
    fn len(&self) -> usize;

    fn get<T>(&'a self, index: usize) -> T
    where
        T: Type<Self::Database>,
        T: Decode<'a, Self::Database>;

    // Returns the value at the `index`; can either be an integer ordinal or a column name.
    // fn get<T, I>(&self, index: I) -> T
    // where
    //     Self::Database: HasSqlType<T>,
    //     I: RowIndex<'a, Self>,
    //     T: Decode<'a, Self::Database>;
}

// A **record** that can be built from a row returned from by the database.
// pub trait FromRow<'a, R>
// where
//     R: Row<'a>,
// {
//     fn from_row(row: R) -> Self;
// }

// #[allow(unused_macros)]
// macro_rules! impl_from_row_for_row {
//     ($R:ty) => {
//         impl crate::row::FromRow<$R> for $R {
//             #[inline]
//             fn from_row(row: $R) -> Self {
//                 row
//             }
//         }
//     };
// }
