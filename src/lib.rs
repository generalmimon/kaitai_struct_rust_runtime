use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Needed {
    Size(usize),
    Unknown,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KError<'a> {
    Incomplete(Needed),
    Encoding { expected: &'static str },
    MissingRoot,
    MissingParent,
    UnexpectedContents { actual: &'a [u8] },
    UnknownVariant(u64),
}
pub type KResult<'a, T> = Result<T, KError<'a>>;

pub trait KStruct<'a>: Default {
    type Parent: KStruct<'a>;
    type Root: KStruct<'a>;

    /// Parse this struct (and any children) from the supplied stream
    fn read<'s: 'a, S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&Self::Root>,
        _parent: Option<&Self::Parent>,
    ) -> KResult<'s, ()>;
}

/// Dummy struct used to indicate an absence of value; needed for
/// root structs to satisfy the associated type bounds in the
/// `KStruct` trait.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct KStructUnit;
impl<'a> KStruct<'a> for KStructUnit {
    type Parent = KStructUnit;
    type Root = KStructUnit;

    fn read<'s: 'a, S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&Self::Root>,
        _parent: Option<&Self::Parent>,
    ) -> KResult<'s, ()> {
        Ok(())
    }
}

pub trait KStream {
    fn is_eof(&self) -> KResult<bool>;
    fn seek(&self, position: u64) -> KResult<()>;
    fn pos(&self) -> KResult<u64>;
    fn size(&self) -> KResult<u64>;

    fn read_s1(&self) -> KResult<i8>;
    fn read_s2be(&self) -> KResult<i16>;
    fn read_s4be(&self) -> KResult<i32>;
    fn read_s8be(&self) -> KResult<i64>;
    fn read_s2le(&self) -> KResult<i16>;
    fn read_s4le(&self) -> KResult<i32>;
    fn read_s8le(&self) -> KResult<i64>;

    fn read_u1(&self) -> KResult<u8>;
    fn read_u2be(&self) -> KResult<u16>;
    fn read_u4be(&self) -> KResult<u32>;
    fn read_u8be(&self) -> KResult<u64>;
    fn read_u2le(&self) -> KResult<u16>;
    fn read_u4le(&self) -> KResult<u32>;
    fn read_u8le(&self) -> KResult<u64>;

    fn read_f4be(&self) -> KResult<f32>;
    fn read_f8be(&self) -> KResult<f64>;
    fn read_f4le(&self) -> KResult<f32>;
    fn read_f8le(&self) -> KResult<f64>;

    fn align_to_byte(&self) -> KResult<()>;
    fn read_bits_int(&self, n: u32) -> KResult<u64>;

    fn read_bytes(&self, len: usize) -> KResult<&[u8]>;
    fn read_bytes_full(&self) -> KResult<&[u8]>;
    fn read_bytes_term(
        &self,
        term: char,
        include: bool,
        consume: bool,
        eos_error: bool,
    ) -> KResult<&[u8]>;

    fn ensure_fixed_contents(&self, expected: &[u8]) -> KResult<&[u8]> {
        let actual = self.read_bytes(expected.len())?;
        if actual == expected {
            Ok(actual)
        } else {
            // Return what the actual contents were; our caller provided us
            // what was expected so we don't need to return it, and it makes
            // the lifetimes way easier
            Err(KError::UnexpectedContents { actual })
        }
    }

    /// Return a byte array that is sized to exclude all trailing instances of the
    /// padding character.
    fn bytes_strip_right(bytes: &[u8], pad: u8) -> &[u8] {
        let mut new_len = bytes.len();
        while new_len > 0 && bytes[new_len - 1] == pad {
            new_len -= 1;
        }
        &bytes[..new_len]
    }

    /// Return a byte array that contains all bytes up until the
    /// termination byte. Can optionally include the termination byte as well.
    fn bytes_terminate(bytes: &[u8], term: u8, include_term: bool) -> &[u8] {
        let mut new_len = 0;
        while bytes[new_len] != term && new_len < bytes.len() {
            new_len += 1;
        }

        if include_term && new_len < bytes.len() {
            new_len += 1;
        }

        &bytes[..new_len]
    }

    // TODO: `process_*` directives
}

#[derive(Default)]
struct BytesReaderState {
    pos: usize,
    bits: u64,
    bits_left: i64,
}
pub struct BytesReader<'a> {
    state: RefCell<BytesReaderState>,
    bytes: &'a [u8],
}
impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BytesReader {
            state: RefCell::new(BytesReaderState::default()),
            bytes,
        }
    }
}
impl<'a> KStream for BytesReader<'a> {
    fn is_eof(&self) -> KResult<bool> {
        unimplemented!()
    }

    fn seek(&self, position: u64) -> KResult<()> {
        unimplemented!()
    }

    fn pos(&self) -> KResult<u64> {
        unimplemented!()
    }

    fn size(&self) -> KResult<u64> {
        unimplemented!()
    }

    fn read_s1(&self) -> KResult<i8> {
        unimplemented!()
    }

    fn read_s2be(&self) -> KResult<i16> {
        unimplemented!()
    }

    fn read_s4be(&self) -> KResult<i32> {
        unimplemented!()
    }

    fn read_s8be(&self) -> KResult<i64> {
        unimplemented!()
    }

    fn read_s2le(&self) -> KResult<i16> {
        unimplemented!()
    }

    fn read_s4le(&self) -> KResult<i32> {
        unimplemented!()
    }

    fn read_s8le(&self) -> KResult<i64> {
        unimplemented!()
    }

    fn read_u1(&self) -> KResult<u8> {
        unimplemented!()
    }

    fn read_u2be(&self) -> KResult<u16> {
        unimplemented!()
    }

    fn read_u4be(&self) -> KResult<u32> {
        unimplemented!()
    }

    fn read_u8be(&self) -> KResult<u64> {
        unimplemented!()
    }

    fn read_u2le(&self) -> KResult<u16> {
        unimplemented!()
    }

    fn read_u4le(&self) -> KResult<u32> {
        unimplemented!()
    }

    fn read_u8le(&self) -> KResult<u64> {
        unimplemented!()
    }

    fn read_f4be(&self) -> KResult<f32> {
        unimplemented!()
    }

    fn read_f8be(&self) -> KResult<f64> {
        unimplemented!()
    }

    fn read_f4le(&self) -> KResult<f32> {
        unimplemented!()
    }

    fn read_f8le(&self) -> KResult<f64> {
        unimplemented!()
    }

    fn align_to_byte(&self) -> KResult<()> {
        unimplemented!()
    }

    // TODO: Clean up the casting nightmare
    fn read_bits_int(&self, n: u32) -> KResult<u64> {
        let bits_needed = n as i64 - self.state.borrow().bits_left;
        if bits_needed > 0 {
            // 1 bit => 1 byte
            // 8 bits => 1 byte
            // 9 bits => 2 bytes
            let bytes_needed = ((bits_needed - 1) / 8) + 1;
            // Need to be careful here, because `read_bytes` will borrow our state as mutable,
            // which panics if we're currently holding a borrow
            // TODO: Return error for bytes_needed > 8
            let buf = self.read_bytes(bytes_needed as usize)?;
            let mut inner = self.state.borrow_mut();
            for b in buf {
                inner.bits <<= 8;
                inner.bits |= *b as u64;
                inner.bits_left += 8;
            }
        }

        let mut inner = self.state.borrow_mut();
        let mut mask = (1u64 << n) - 1;
        let shift_bits = inner.bits_left - n as i64;
        mask <<= shift_bits;

        let result: u64 = (inner.bits & mask) >> shift_bits;

        inner.bits_left -= n as i64;
        mask = (1u64 << inner.bits_left) - 1;
        inner.bits &= mask;

        Ok(result)
    }

    fn read_bytes(&self, len: usize) -> KResult<&[u8]> {
        let cur_pos = self.state.borrow().pos;
        if len + cur_pos > self.bytes.len() {
            return Err(KError::Incomplete(Needed::Size(
                len + cur_pos - self.bytes.len(),
            )));
        }

        self.state.borrow_mut().pos += len;
        Ok(&self.bytes[cur_pos..cur_pos + len])
    }

    fn read_bytes_full(&self) -> KResult<&[u8]> {
        unimplemented!()
    }

    fn read_bytes_term(
        &self,
        term: char,
        include: bool,
        consume: bool,
        eos_error: bool,
    ) -> KResult<&[u8]> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_strip_right() {
        let b = [1, 2, 3, 4, 5, 5, 5, 5];
        let c = BytesReader::bytes_strip_right(&b, 5);

        assert_eq!([1, 2, 3, 4], c);
    }

    #[test]
    fn basic_read_bytes() {
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bytes(4).unwrap(), &[1, 2, 3, 4]);
        assert_eq!(reader.read_bytes(3).unwrap(), &[5, 6, 7]);
        assert_eq!(
            reader.read_bytes(4).unwrap_err(),
            KError::Incomplete(Needed::Size(3))
        );
        assert_eq!(reader.read_bytes(1).unwrap(), &[8]);
    }

    #[test]
    fn read_bits_single() {
        let b = vec![0x80];
        let mut reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_multiple() {
        // 0xA0
        let b = vec![0b10100000];
        let mut reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
        assert_eq!(reader.read_bits_int(1).unwrap(), 0);
        assert_eq!(reader.read_bits_int(1).unwrap(), 1);
    }

    #[test]
    fn read_bits_large() {
        let b = vec![0b10100000];
        let mut reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(3).unwrap(), 5);
    }

    #[test]
    fn read_bits_span() {
        let b = vec![0x01, 0x80];
        let mut reader = BytesReader::new(&b[..]);

        assert_eq!(reader.read_bits_int(9).unwrap(), 3);
    }
}
