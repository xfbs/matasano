# Matasano

The [cryptopals](http://cryptopals.com) crypto challenges, implemented in
[Rust](https://rust-lang.org/).

## Table of Content

- [Set 1: Basics](#set-1-basics)
    - [Convert Hex to Base64](#convert-hex-to-base64)

## Set 1: [Basics](http://cryptopals.com/sets/1)

> This is the qualifying set. We picked the exercises in it to ramp developers
> up gradually into coding cryptography, but also to verify that we were
> working with people who were ready to write code.
>
> This set is relatively easy. With one exception, most of these exercises
> should take only a couple minutes. But don't beat yourself up if it takes
> longer than that. It took Alex two weeks to get through the set!
>
> If you've written any crypto code in the past, you're going to feel like
> skipping a lot of this. Don't skip them. At least two of them (we won't say
> which) are important stepping stones to later attacks.

### Convert Hex to Base64

> The string:
>   
>     49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
>
> Should produce:
>   
>     SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
>
> So go ahead and make that happen. You'll need to use this code for the rest
> of the exercises.

This problem can easily be broken down into two subproblems: *hex-decoding* and
*base64-encoding*. Let's start with the hex format.

#### Hex Conversion

Hexadecimal, being a base-16 number system, has one very useful property: one
byte (which is an eight-digit binary number) representable as a two-digit
hexadecimal number, and vice versa. That means that for every byte, we get two
hexadecimal numbers, and for every two hexadecimal numbers, we get one byte.
This means that one hexadecimal number is worth one half-byte, also known as nibble.

So, in order to convert a byte into hexadecimal, there are two steps: turn
the byte into two nibbles, lower and higher, and then hex-encode both.

Extracting both nibbles of a file is easy with a bit of bit-banging, yielding
a tuple of the higher and lower nibbles. Each nibble is of course only 4 bits wide,
but represented as u8, for lack of a smaller data type in Rust.

###### [src/nibble.rs](src/nibble.rs), lines 1-4

```rust
/// Extract both nibbles of a byte.
pub fn nibbles(byte: u8) -> (u8, u8) {
    ((byte >> 4) & 0b1111, byte & 0b1111)
}
```

With this, and the library function `std::char::from_digit()`, it is easy to implement
a function to turn a single byte (`u8` in Rust) into hexadecimal notation (two `char`s).
Since the nibbles are each only 4 bit wide, the `char::from_digit(num: u32, radix: u32)`
call is never going to fail, so it's safe to call `unwrap()` on the result and call it
a day.

###### [src/hex.rs](src/hex.rs), lines 5-9

```rust
/// Hex-encodes a single byte.
pub fn to_hex(byte: u8) -> (char, char) {
    let (msn, lsn) = nibbles(byte);
    (from_digit(msn as u32, 16).unwrap(), from_digit(lsn as u32, 16).unwrap())
}
```

But this isn't the end of it. Most of the time, there is more than just a single byte
to be converted. Rust provides a useful `Iterator` interface to working with streams
of data. So, to convert any stream of bytes into hex-encoded data, an Iterator adapter
comes in handy.

This looks a bit intimidating due to the unwiedly return type, but it works. For every
byte, it uses the `to_hex()` function with `Iterator::flat_map()` to map each byte to
two `char`s representing that byte.

###### [src/hex.rs](src/hex.rs), lines 11-22

```rust
pub trait ToHex: Iterator<Item=u8> where Self: Sized {
    /// Taking an Iterator over `u8`, generate a new Iterator over the `u8` data hex-encoded,
    /// as `char`s.
    fn to_hex(self) -> FlatMap<Self, Chain<Once<char>, Once<char>>, fn(u8) -> Chain<Once<char>, Once<char>>> {
        self.flat_map(|byte: u8| {
            let (msn, lsn) = to_hex(byte);
            once(msn).chain(once(lsn))
        })
    }
}

impl<T> ToHex for T where T: Iterator<Item=u8> {}
```

So far, any arbitary data can be hex-encoded with `to_hex()` on any
iterator that iterates over `u8`s, like so:

```rust
assert_eq!(
    "astringent".bytes().to_hex().collect::<String>(),
    String::from("61737472696e67656e74"));
```

But what about going the other way, from hex-encoded back into raw data? This
direction is just a tad more difficult, because there are more constraints and
error handling. 

Every possible byte in a raw data stream can be hex-encoded, so error handling
isn't necessary. But when taking data and hex-decoding it into bytes, there are
two potential issues: first, not ever character is a valid digit in hexadecimal
notation. Additionally, for every byte, two hexadecimal digits are needed,
meaning that there is an error when the input stream has an odd length.


