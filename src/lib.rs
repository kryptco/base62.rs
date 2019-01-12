// Copyright 2019 KryptCo, Inc
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

/*!
A library for encoding/decoding byte arrays to/from a base62 strings.

# Alphabet
This library defines the Base62 alphabet as the following characters:

```0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz```

# How it works
A byte array (leading zeros allowed) is prepended with `0x01` and is treated
as a big-endian unsigned integer (`num_bigint::BigUint`).

This number is repeatedly divided by our base, `62`, and each remainder is used as an index into our alphabet above,
producing the base62 encoded string.

To decode, we run the algorithm above in reverse.

# Example

```rust

fn main() {
    let input = vec![0xDE,0xAD,0xBE,0xEF];
    let encoded = base_62::encode(&input);
    println!("0xDEADBEEF = {}", encoded);
    let deadbeef = base_62::decode("JsoUl8").unwrap();

    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt";
    let encoded = base_62::encode(input.as_bytes());
    println!("lorem... = {}", encoded);
    let loremipsum = base_62::decode("Inj62xrWzFT5RgFoP72ZkfbrMabXdyZeYGijtTt8zuBN4XvHvEw6x2pk2BtdepGle57axcSeY2ixeXqOvwpE2VaEE3pHeeumHvIbZf0qUUxRBg99NrIALFCE").unwrap();
}
```
*/

pub mod base62;
pub use self::base62::*;