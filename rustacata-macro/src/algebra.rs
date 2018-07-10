#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro2;

use proc_macro2::{Span, TokenStream};

use input::{Args, Data};

pub fn generate(args: Args, input: Data) -> TokenStream {

}

fn gen_variant(item: syn::ItemEnum, variant: syn::Variant) -> TokenStream {
    let name = Ident::new(&format!("tr_{}", variant.ident), Span::call_site());

    let a_lf = syn::Lifetime::new("'a", Span::call_site());
    let b_lf = syn::Lifetime::new("'b", Span::call_site());

    let self_ty = item.ty;
    let self_tr = quote! { Transformer<& #b_lf #self_ty, R, Env> };

    let fields = variant.fields.iter().map(|field| quote! { & #b_lf field.ty });

    quote! {
        #name : Box<#a_lf + for<#b_lf> Fn(#self_tr, Env, #(#fields),*) -> R>
    }
}