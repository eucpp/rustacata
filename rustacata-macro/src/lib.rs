#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use syn::{Item, ItemEnum, Variant, Ident};
use proc_macro2::{Span, TokenStream};

mod input;
mod ftable;

#[proc_macro_attribute]
pub fn cata(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = input::Args::parse(args);
    let data = input::Datatype::parse(input);

    let alg = ftable::generate(&args, &data);

    let expanded = quote! {
        #data

        #alg
    };

    expanded.into()
}