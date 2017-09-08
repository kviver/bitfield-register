#![feature(proc_macro)]

extern crate proc_macro;
// extern crate syn;

// #[macro_use]
// extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn register(_: TokenStream, input: TokenStream) -> TokenStream {
    return input;
}