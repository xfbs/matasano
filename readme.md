# Matasano

The [cryptopals](http://cryptopals.com) crypto challenges, implemented in
[Rust](https://rust-lang.org/).

## Table of Content

- [Set 1: Basics](#set-1-basics)
    - [Convert Hex to Base64](#convert-hex-to-base64)

## Set 1: Basics

>   This is the qualifying set. We picked the exercises in it to ramp
>   developers up gradually into coding cryptography, but also to verify that
>   we were working with people who were ready to write code.
>
>   This set is relatively easy. With one exception, most of these exercises
>   should take only a couple minutes. But don't beat yourself up if it takes
>   longer than that. It took Alex two weeks to get through the set!
>
>   If you've written any crypto code in the past, you're going to feel like
>   skipping a lot of this. Don't skip them. At least two of them (we won't say
>   which) are important stepping stones to later attacks.

### Convert Hex to Base64

>   The string:
>   
>       49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
>
>   Should produce:
>   
>       SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
>
>   So go ahead and make that happen. You'll need to use this code for the rest of
>   the exercises.
>
>   >   **Cryptopals Rule**: Always operate on raw bytes, never on encoded
>   >   strings. Only use hex and base64 for pretty-printing.

Writing a function to convert from hex to base64 would solve this problem, but that's not
the one true path of the programmer. A better solution is to take this problem, and divide it
up into smaller, modular subproblems.

In this case, there are two embedded problems: decoding hex-encoded data, and base64-encoding
data.

