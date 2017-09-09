# bitfield-register
Rust bitfield library for low-level registers


# usage
```rust
#![feature(proc_macro)]

extern crate bitfield_register;
use bitfield_register::register;

#[derive(Debug)]
enum RW {
    R = 0,
    W = 1
}

impl From<u8> for RW {
    fn from(value: u8) -> Self {
        return match value {
            0 => RW::R,
            1 => RW::W,
            _ => unreachable!()
        }
    }
}

#[register()]
struct Test {
    #[bitfield(from=1, to=2)]
    foo: u8,
    #[bitfield(at=2)]
    bar: RW
}


fn main() {
    let test = Test(42);

    println!("foo {}", test.0);
    println!("var {:?}", test.bar());
}
```