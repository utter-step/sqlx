use crate::io::{Buf, ByteStr};
use crate::postgres::protocol::Decode;
use byteorder::NetworkEndian;
use core::marker::PhantomData;
use std::fmt::{self, Debug};
use std::ops::Range;

pub struct DataRow<'c> {
    phantom: PhantomData<&'c mut ()>,

    //    buffer: Vec<[u8]>,
    values: Box<[Option<Range<u32>>]>,

    buffer: &'c Vec<u8>,
}

impl<'c> DataRow<'c> {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&'c self, index: usize) -> Option<&'c [u8]> {
        let range = self.values[index].as_ref()?;

        Some(&self.buffer[(range.start as usize)..(range.end as usize)])
    }
}

impl<'c> DataRow<'c> {
    pub(crate) fn decode(mut buf: &[u8], work: &'c mut Vec<u8>) -> crate::Result<Self> {
        let len = buf.get_u16::<NetworkEndian>()? as usize;
        // let buffer: Box<[u8]> = buf.into();
        work.clear();
        work.extend_from_slice(buf);
        let mut values = Vec::with_capacity(len);
        let mut index = 6;

        while values.len() < len {
            // The length of the column value, in bytes (this count does not include itself).
            // Can be zero. As a special case, -1 indicates a NULL column value.
            // No value bytes follow in the NULL case.
            let size = buf.get_i32::<NetworkEndian>()?;

            if size == -1 {
                values.push(None);

                index += 4;
            } else {
                values.push(Some((index)..(index + (size as u32))));

                index += (size as u32) + 4;
                buf.advance(size as usize);
            }
        }

        Ok(Self {
            phantom: PhantomData,
            values: values.into_boxed_slice(),
            buffer: work,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{DataRow, Decode};

    const DATA_ROW: &[u8] = b"\0\x03\0\0\0\x011\0\0\0\x012\0\0\0\x013";

    #[test]
    fn it_decodes_data_row() {
        let m = DataRow::decode(DATA_ROW).unwrap();

        assert_eq!(m.values.len(), 3);

        assert_eq!(m.get(0), Some(&b"1"[..]));
        assert_eq!(m.get(1), Some(&b"2"[..]));
        assert_eq!(m.get(2), Some(&b"3"[..]));
    }
}
