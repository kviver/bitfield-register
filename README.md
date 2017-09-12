# bitfield-register
Rust bitfield library for low-level registers


# usage
```rust
#![feature(proc_macro)]

extern crate bitfield_register;
use bitfield_register::register;

use std::convert::*;

#[derive(Debug)]
enum RW {R, W}

impl From<[u8;1]> for RW { fn from(value: [u8;1]) -> Self {
    return match value[0] {
        0 => RW::R,
        1 => RW::W,
        _ => unreachable!()
    }
}}
impl Into<[u8;1]> for RW { fn into(self) -> [u8;1] {
    return match self {
        RW::R => [0],
        RW::W => [1]
    }
}}


#[derive(Debug)]
struct Address(u16);

impl From<[u8;2]> for Address { fn from(value: [u8;2]) -> Self {
    println!("{:?}", value);
    return Address(((value[1] as u16) << 8) + value[0] as u16);
}}
impl Into<[u8;2]> for Address { fn into(self) -> [u8;2] {
    return [(self.0 & 0xFF) as u8, ((self.0 & (0xFF << 8)) >> 8) as u8];
}}

#[register()]
struct Test {
    #[bitfield(from=1, to=10)]
    address: Address,
    #[bitfield(at=14)]
    rw: RW
}


fn main() {

    let mut test: Test = Default::default();

    test.set_address(Address(1023));

    test.set_rw(RW::W);

    println!("raw value:");
    for i in 0..test.0.len() {
        print!("{:0>8b} ", test.0[test.0.len() - i - 1]);
    }
    println!();

    println!("address: {:?}", test.get_address());
    println!("rw: {:?}", test.get_rw());
}
```