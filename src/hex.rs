use std::char::from_digit;

/// Iterator adaptor that hex-encodes data.
pub struct ToHexIter<T: Iterator> {
    input: T,
    store: Option<char>
}

impl<T: Iterator> ToHexIter<T> {
    /// Create a new `ToHexIter` from an `Iterator`.
    pub fn new(iter: T) -> Self {
        ToHexIter {
            input: iter,
            store: None
        }
    }
}

/// Hex-encodes a single byte.
///
/// FIXME: how about a generic interface for turning T into
/// an n-tuple of hex-encoded data?
///
/// ## Examples
///
/// ```
/// assert_eq!(matasano::to_hex(165), ('a', '5'));
/// ```
pub fn to_hex(byte: u8) -> (char, char) {
    // split first four and last four bits
    let first  = byte >> 4;
    let second = byte & 0b00001111;

    (from_digit(first as u32, 16).unwrap(), from_digit(second as u32, 16).unwrap())
}

#[test]
fn test_to_hex() {
    assert!(to_hex(  0) == ('0', '0'));
    assert!(to_hex( 15) == ('0', 'f'));
    assert!(to_hex( 16) == ('1', '0'));
    assert!(to_hex(160) == ('a', '0'));
    assert!(to_hex(175) == ('a', 'f'));
    assert!(to_hex(255) == ('f', 'f'));
}

impl<T: Iterator<Item=u8>> Iterator for ToHexIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let store = self.store;
        match store {
            Some(val) => {
                self.store = None;
                Some(val)
            },
            None => {
                let next = self.input.next();
                match next {
                    None => None,
                    Some(val) => {
                        let (first, second) = to_hex(val);
                        self.store = Some(second);
                        Some(first)
                    }
                }
            }
        }
    }
}

/// Turn an arbitrary Iterator into a ToHexIter.
pub trait ToHex: Iterator {
    fn to_hex(self) -> ToHexIter<Self> where Self: Sized {
        ToHexIter::new(self)
    }
}

impl<T> ToHex for T where T: Iterator {}

#[test]
fn test_to_hex_iter() {
    assert_eq!("abcdefg".bytes().to_hex().collect::<String>(), String::from("61626364656667"));
}
