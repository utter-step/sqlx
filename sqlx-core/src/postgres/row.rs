use core::marker::PhantomData;

use std::collections::HashMap;
use std::sync::Arc;

use crate::decode::Decode;
use crate::postgres::protocol::DataRow;
use crate::postgres::Postgres;
use crate::row::Row;
use crate::types::Type;

pub struct PgRow<'c> {
    pub(super) data: DataRow<'c>,
    pub(super) columns: Arc<HashMap<Box<str>, usize>>,
}

impl<'c> Row<'c> for PgRow<'c> {
    type Database = Postgres;

    fn len(&self) -> usize {
        //        self.data.len()

        todo!()
    }

    fn try_get<'de: 'c, T>(&'de self, index: usize) -> crate::Result<T>
    where
        T: Type<Self::Database>,
        T: Decode<'de, Self::Database>,
    {
        // T::decode(self.data.get(index))
        todo!()
    }
}

// TODO: Restore [RowIndex]

//impl RowIndex<PgRow> for usize {
//    fn try_get<T>(&self, row: &PgRow) -> crate::Result<T>
//    where
//        <PgRow as Row>::Database: HasSqlType<T>,
//        T: Decode<<PgRow as Row>::Database>,
//    {
//        Ok(Decode::decode_nullable(row.data.get(*self))?)
//    }
//}
//
//impl RowIndex<PgRow> for &'_ str {
//    fn try_get<T>(&self, row: &PgRow) -> crate::Result<T>
//    where
//        <PgRow as Row>::Database: HasSqlType<T>,
//        T: Decode<<PgRow as Row>::Database>,
//    {
//        let index = row
//            .columns
//            .get(*self)
//            .ok_or_else(|| crate::Error::ColumnNotFound((*self).into()))?;
//        let value = Decode::decode_nullable(row.data.get(*index))?;
//
//        Ok(value)
//    }
//}
//
//impl_from_row_for_row!(PgRow);
