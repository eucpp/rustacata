#![feature(proc_macro)]

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use syn::{Item, ItemEnum, Variant, Ident};
use proc_macro2::{Span, TokenStream};

mod input;
mod algebra;
mod traverse;
mod catamorphism;
mod foldable;
mod utils;

use foldable::Foldable;
use traverse::BorrowTraverse;
use catamorphism::Catamorphism;

#[proc_macro_attribute]
pub fn cata(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = input::Args::parse(args);
    let dt = input::Datatype::parse(input);

    let alg = Foldable::new();
    let trv = BorrowTraverse::new();
    let cata = Catamorphism::new(alg, trv, dt).codegen();


    let expanded = quote! {
        #cata
    };

    expanded.into()
}