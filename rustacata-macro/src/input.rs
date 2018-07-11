extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;
extern crate quote;

use syn::{Item, ItemStruct, ItemEnum, Variant, Ident};
use quote::{ToTokens};

pub struct Args(());

impl Args {
    pub fn parse(input: proc_macro::TokenStream) -> Args {
        Args(())
    }
}

pub enum Data {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum),
}

impl Data {
    pub fn parse(input: proc_macro::TokenStream) -> Data {
        let input : Item = syn::parse(input).unwrap();
        match input {
            Item::Enum(item) => Data::Enum(item),
            _ => unimplemented!(),
        }
    }
}

impl ToTokens for Data {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Data::Enum(ref item) => item.to_tokens(tokens),
            _ => unimplemented!(),
        }
    }
}