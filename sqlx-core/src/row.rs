//! Contains the Row and FromRow traits.

use crate::database::Database;
use crate::decode::Decode;
use crate::types::Type;

//pub trait RowIndex<R: ?Sized>
//where
//    R: Row,
//{
//    fn try_get<T>(&self, row: &R) -> crate::Result<T>
//    where
//        R::Database: HasSqlType<T>,
//        T: Decode<R::Database>;
//}

pub trait Row<'c>: Unpin + Send {
    type Database: Database;

    fn get<T>(&self, index: usize) -> T
    where
        T: Type<Self::Database>,
        T: Decode<'c, Self::Database>;
}
