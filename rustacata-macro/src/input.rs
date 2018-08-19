extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;
extern crate quote;

use std::iter::{Iterator};

use syn::{Item, ItemStruct, ItemEnum, Variant, Ident, Type, TypeParam};
use quote::{ToTokens};

pub struct Args(());

impl Args {
    pub fn parse(input: proc_macro::TokenStream) -> Args {
        Args(())
    }
}

pub enum Datatype {
    Struct(syn::ItemStruct),
    Enum(syn::ItemEnum),
}

impl Datatype {
    pub fn parse(input: proc_macro::TokenStream) -> Datatype {
        println!("Here!!!");
        let input : Item = syn::parse(input).unwrap();
        println!("There!!!");
        match input {
            Item::Enum(item) => Datatype::Enum(item),
            _ => unimplemented!(),
        }
    }

    pub fn ident(&self) -> Ident {
        match *self {
            Datatype::Enum(ref item) => item.ident.clone(),
            _ => unimplemented!(),
        }
    }

    pub fn ty(&self) -> Type {
        let ident = self.ident();
        parse_quote! { #ident }
    }

    pub fn type_params(&self) -> impl Iterator<Item = &TypeParam> {
        match self {
            Datatype::Enum(ref item) => item.generics.type_params(),
            _ => unimplemented!(),
        }
    }
}

impl ToTokens for Datatype {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Datatype::Enum(ref item) => item.to_tokens(tokens),
            _ => unimplemented!(),
        }
    }
}