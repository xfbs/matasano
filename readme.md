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

So, in order to convert a byte into hexadecimal, there are two steps: we need to turn
the byte into two nibbles, lower and higher, and then we need to hex-encode both.

> File [src/nibble.rs](src/nibble.rs), lines 1-4:
> ```rust
> /// Extract both nibbles of a byte.
> pub fn nibbles(byte: u8) -> (u8, u8) {
>     ((byte >> 4) & 0b1111, byte & 0b1111)
> }
> ```
