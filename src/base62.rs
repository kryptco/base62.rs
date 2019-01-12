// Copyright 2019 KryptCo, Inc
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use num_bigint::{BigUint};
use num_traits::{Zero, One, ToPrimitive};
use num_integer::Integer;
use failure::Fail;

/// Convert a byte buffer into a Base62 String
pub fn encode(bytes: &[u8]) -> String {
    if bytes.is_empty() { return "".into() }

    let mut input = vec![1u8];
    input.extend_from_slice(bytes);

    let mut result = String::new();
    let mut val = BigUint::from_bytes_be(&input);
    let base:BigUint = (BASE.to_owned() as u64).into();

    while val > BigUint::zero() {
        let remainder = val.mod_floor(&base).to_usize().unwrap_or(0);
        result.push(ALPHABET[remainder]);
        val /= &base;
    }

    result
}

/// Convert a Base62 String into a byte buffer
pub fn decode(input: &str) -> Result<Vec<u8>, Error> {
    let mut val:BigUint = BigUint::zero();
    let mut base_mul = BigUint::one();
    let base:BigUint = (BASE.to_owned() as u64).into();

    for c in input.chars().into_iter() {
        let remainder:BigUint = char_to_remainder(c)?.into();
        val += remainder*&base_mul;
        base_mul *= &base;
    }

    Ok(val.to_bytes_be()[1..].to_vec())
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid character '{}'", character)]
    BadCharacter { character: char}
}

// Alphabet Mapping
const BASE: usize = 62;
const ALPHABET: [char; BASE] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z',
];

/// Reverse map a char to its "remainder" (its index in `ALPHABET`)
fn char_to_remainder(c: char) -> Result<u64, Error> {
    let i = match c {
        '0'...'9' => ((c as u64) % ('0' as u64)),
        'A'...'Z' => ((c as u64) % ('A' as u64)) + 10,
        'a'...'z' => ((c as u64) % ('a' as u64)) + 36,
        _ => return Err(Error::BadCharacter { character: c })
    };

    Ok(i)
}

#[test]
fn test_encode() {
    let cases:Vec<Vec<u8>> = vec![
        vec![],
        vec![0u8],
        vec![1u8],
        vec![0u8,0u8],
        vec![0u8,1u8],
        vec![1u8,0u8],
        vec![1u8,1u8],
        vec![0, 0, 0, 1],
        [62u8; 10].to_vec(),
        [63u8; 10].to_vec(),
        [1u8; 10].to_vec(),
        [0u8; 10].to_vec(),
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt".as_bytes().to_vec(),
        vec![0xDE, 0xAD, 0xBE, 0xEF],
    ];

    cases.into_iter().enumerate().for_each(|(case, input)| {
        let encoded = encode(&input);
        let decoded = decode(&encoded).expect("error decoding base62 input");
        assert_eq!(input, decoded, "\nCase {}:\nbase62({})\nmismatch: \nencoded({:?}) != \ndecoded({:?})", case, encoded, input, decoded);
    })
}

#[test]
fn test_invalid() {
    decode("abc-").expect_err("expected invalid '-'");
    decode("wSBzv9UB5PeI/26").expect_err("expected invalid '/'");
    decode("jSO+uL8").expect_err("expected invalid '+");
}