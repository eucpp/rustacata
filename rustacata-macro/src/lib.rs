#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use syn::{Item, ItemEnum, Variant, Ident};
use proc_macro::TokenStream;
use proc_macro2::Span;

#[proc_macro_attribute]
pub fn derive_transformer(args: TokenStream, input: TokenStream) -> TokenStream {
    let input : Item = syn::parse(input).unwrap();

    match input {
        Item::Enum(ref enum_item) => {
            let transformation = Ident::new(
                &format!("{}Transformation", enum_item.ident),
                Span::call_site()
            );
            let methods = enum_item.variants.iter()
                .map(|variant| derive_variant_method(enum_item, variant));

            let expanded = quote! {
                #input

                trait #transformation {
                    type Inh;
                    type Synth;

                    #(#methods;)*
                }
            };

            expanded.into()
        },
        _ => unimplemented!()
    }
}

fn derive_variant_method(enum_item: &ItemEnum, variant: &Variant) -> proc_macro2::TokenStream {
    let enum_name = Ident::new(&format!("{}", enum_item.ident), Span::call_site());
    let meth_name = Ident::new(&format!("fold_{}", variant.ident).to_lowercase(), Span::call_site());

    let mut cnt = 0;
    let args = variant.fields.iter().map(|field| {
        let name = Ident::new(&format!("x{}", cnt), Span::call_site());
        let ty = &field.ty;
        cnt = cnt + 1;
        quote! { #name : & #ty }
    });

    quote! {
        fn #meth_name <Tr: Transformer<#enum_name, Self::Inh, Self::Synth> >(
            tr: &Tr,
            inh: Self::Inh,
            #(#args),*
        ) -> Self::Synth
    }
}