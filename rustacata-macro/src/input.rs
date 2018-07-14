extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;
extern crate quote;

use syn::{Item, ItemStruct, ItemEnum, Variant, Ident, Type};
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
        let input : Item = syn::parse(input).unwrap();
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
}

impl ToTokens for Datatype {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Datatype::Enum(ref item) => item.to_tokens(tokens),
            _ => unimplemented!(),
        }
    }
}