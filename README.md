# bitfield-register
Rust bitfield library for low-level registers


# usage
```rust
#![feature(proc_macro)]

extern crate bitfield_register;
use bitfield_register::register;

#[register()]
struct Test {
    foo: u8
}
```