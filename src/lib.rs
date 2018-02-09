#![feature(try_from)]
mod hex;
mod bytes;

pub use hex::{ToHex};
pub use bytes::ToBytes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
