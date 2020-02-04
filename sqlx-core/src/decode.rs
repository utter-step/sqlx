//! Types and traits for decoding values from the database.

use crate::database::{Database, HasRawValue};

pub trait Decode<'de, DB>: Sized
where
    DB: Database + ?Sized,
{
    fn decode(raw: <DB as HasRawValue<'de>>::RawValue) -> crate::Result<Self>;
}

//impl<T, DB> Decode<DB> for Option<T>
//where
//    DB: Database + HasSqlType<T>,
//    T: Decode<DB>,
//{
//    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
//        T::decode(buf).map(Some)
//    }
//
//    fn decode_null() -> Result<Self, DecodeError> {
//        Ok(None)
//    }
//}
//
//impl fmt::Debug for DecodeError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        f.write_str("DecodeError(")?;
//
//        match self {
//            DecodeError::UnexpectedNull => write!(f, "unexpected null for non-null column")?,
//            DecodeError::Message(err) => write!(f, "{}", err)?,
//            DecodeError::Other(err) => write!(f, "{:?}", err)?,
//        }
//
//        f.write_str(")")
//    }
//}
//
//impl fmt::Display for DecodeError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            DecodeError::UnexpectedNull => f.write_str("unexpected null for non-null column"),
//            DecodeError::Message(err) => write!(f, "{}", err),
//            DecodeError::Other(err) => write!(f, "{}", err),
//        }
//    }
//}
//
//impl<E> From<E> for DecodeError
//where
//    E: StdError + Send + Sync + 'static,
//{
//    fn from(err: E) -> DecodeError {
//        DecodeError::Other(Box::new(err))
//    }
//}
