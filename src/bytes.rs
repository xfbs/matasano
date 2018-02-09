use std::mem::size_of;
use std::ops::{Shr, BitAnd};
use std::convert::TryFrom;

/// Iterator over the nibbles in a number.
#[derive(Debug, PartialEq, Clone)]
pub struct Bytes<T> {
    item: T,
    pos: usize
}

#[test]
fn test_bytes_struct() {
    for num in 0..100 {
        assert_eq!(Bytes { item: num, pos: 0 }.item, num);
        assert_eq!(Bytes { item: 0, pos: num }.pos, num);
    }
}

/// Convert a number to an iterator over it's nibbles.
pub trait ToBytes where Self: Sized {
    /// Get iterator over bytes.
    fn bytes(self) -> Bytes<Self> {
        Bytes {
            item: self,
            pos: size_of::<Self>() * BYTE_WIDTH
        }
    }
}

impl<T: Sized> ToBytes for T {}

#[test]
fn test_to_bytes_trait() {
    for num in 0..256 {
        assert_eq!((num as u8 ).bytes(), Bytes { item: num as u8 , pos: 8  });
        assert_eq!((num as u16).bytes(), Bytes { item: num as u16, pos: 16 });
        assert_eq!((num as u32).bytes(), Bytes { item: num as u32, pos: 32 });
        assert_eq!((num as u64).bytes(), Bytes { item: num as u64, pos: 64 });

        assert_eq!((num as i8 ).bytes(), Bytes { item: num as i8 , pos: 8  });
        assert_eq!((num as i16).bytes(), Bytes { item: num as i16, pos: 16 });
        assert_eq!((num as i32).bytes(), Bytes { item: num as i32, pos: 32 });
        assert_eq!((num as i64).bytes(), Bytes { item: num as i64, pos: 64 });
    }
}

/// Width of a byte, in bits.
const BYTE_WIDTH: usize = 8;

/// Mask to extract the least significant byte from a number.
const BYTE_MASK: u8 = ((1u16 << BYTE_WIDTH) - 1) as u8;

impl<T> Iterator for Bytes<T> where
    T: Copy + Shr<usize, Output=T> + BitAnd<T, Output=T>,
    u8: TryFrom<T>,
    u8: Into<T>
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            0 => None,
            _ => {
                self.pos -= BYTE_WIDTH;
                let shift = self.pos;
                let shifted = self.item >> shift;
                let mask = BYTE_MASK.into();
                let byte = shifted & mask;
                Some(u8::try_from(byte).ok().unwrap())
            }
        }
    }
}

#[test]
fn test_bytes_iter() {
    let mut bytes_u8 = 0x3Fu8.bytes();
    assert_eq!(bytes_u8.next(), Some(0x3F));
    assert_eq!(bytes_u8.next(), None);

    let mut bytes_u16 = 0x59FAu16.bytes();
    assert_eq!(bytes_u16.next(), Some(0x59));
    assert_eq!(bytes_u16.next(), Some(0xFA));
    assert_eq!(bytes_u16.next(), None);

    let mut bytes_u32 = 0xDEADBEEFu32.bytes();
    assert_eq!(bytes_u32.next(), Some(0xDE));
    assert_eq!(bytes_u32.next(), Some(0xAD));
    assert_eq!(bytes_u32.next(), Some(0xBE));
    assert_eq!(bytes_u32.next(), Some(0xEF));
    assert_eq!(bytes_u32.next(), None);

    let mut bytes_u64 = 0xDEADBEEFCAFEBABEu64.bytes();
    assert_eq!(bytes_u64.next(), Some(0xDE));
    assert_eq!(bytes_u64.next(), Some(0xAD));
    assert_eq!(bytes_u64.next(), Some(0xBE));
    assert_eq!(bytes_u64.next(), Some(0xEF));
    assert_eq!(bytes_u64.next(), Some(0xCA));
    assert_eq!(bytes_u64.next(), Some(0xFE));
    assert_eq!(bytes_u64.next(), Some(0xBA));
    assert_eq!(bytes_u64.next(), Some(0xBE));
    assert_eq!(bytes_u64.next(), None);
}
