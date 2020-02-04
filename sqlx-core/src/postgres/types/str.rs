use std::str;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::types::PgTypeInfo;
use crate::types::Type;
use crate::Postgres;

impl Type<Postgres> for str {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::TEXT)
    }
}

impl Type<Postgres> for [&'_ str] {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::ARRAY_TEXT)
    }
}

impl Type<Postgres> for String {
    fn type_info() -> PgTypeInfo {
        <str as Type<Postgres>>::type_info()
    }
}

impl Encode<Postgres> for str {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }

    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl Encode<Postgres> for String {
    fn encode(&self, buf: &mut Vec<u8>) {
        <str as Encode<Postgres>>::encode(self.as_str(), buf)
    }

    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<'de> Decode<'de, Postgres> for &'de str {
    fn decode(buf: Option<&'de [u8]>) -> crate::Result<Self> {
        todo!()

        // str::from_utf8(buf).map_err(Into::into)
    }
}

impl<'de> Decode<'de, Postgres> for String {
    fn decode(buf: Option<&'de [u8]>) -> crate::Result<Self> {
        todo!()

        // <&str as Decode<Postgres>>::decode(buf).map(|s| s.to_owned())
    }
}
