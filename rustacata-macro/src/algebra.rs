//#[macro_use]
//extern crate syn;
//
//#[macro_use]
//extern crate quote;

extern crate proc_macro2;

use syn::{Ident, ItemEnum, Variant, Lifetime};

use proc_macro2::{Span, TokenStream};

use input::{Args, Data};

pub fn generate(args: &Args, input: &Data) -> TokenStream {
    match input {
        Data::Enum(ref item) => gen_enum(args, item),
        _ => unimplemented!(),
    }
}

fn gen_enum(args: &Args, item: &ItemEnum) -> TokenStream {
    let name = algebra_name(&item.ident);
    let fields = item.variants.iter().map(|variant| gen_variant_field(&item, &variant));
    let setters = item.variants.iter().map(|variant| gen_variant_setter(&item, &variant));
    quote! {
        struct #name<'a, R, Env> {
            #(#fields),*
        }

        impl<'a, R, Env> #name<'a, R, Env> {
            #(#setters)*
        }
    }
}

fn gen_variant_field(item: &ItemEnum, variant: &Variant) -> TokenStream {
    let field_name = variant_field_name(&variant.ident);

    let fn_ty = variant_fn_type(item, variant);

    quote! {
        #field_name : Box<'a + #fn_ty>
    }
}

fn gen_variant_setter(item: &ItemEnum, variant: &Variant) -> TokenStream {
    let alg_name = algebra_name(&item.ident);
    let field_name = variant_field_name(&variant.ident);
    let setter_name = variant_setter_name(&variant.ident);

    let fn_ty = variant_fn_type(item, variant);

    quote! {
        fn #setter_name <'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + #fn_ty
        {
            #alg_name { #field_name: Box::new(f), ..self }
        }
    }
}

fn algebra_name(ident: &Ident) -> Ident{
    Ident::new(&format!("{}Algebra", *ident), Span::call_site())
}

fn variant_field_name(ident: &Ident) -> Ident {
    Ident::new(&format!("tr_{}", *ident), Span::call_site())
}

fn variant_setter_name(ident: &Ident) -> Ident {
    Ident::new(&format!("with_tr_{}", *ident), Span::call_site())
}

fn variant_fn_type(item: &ItemEnum, variant: &Variant) -> TokenStream {
    let self_ty = &item.ident;
    let self_tr = quote! { Transformer<& 'b #self_ty, R, Env> };

    let args = variant.fields.iter().map(|field| {
        let field_ty = &field.ty;
        quote! { & 'b #field_ty }
    });

    quote! {
        for<'b> Fn(#self_tr, Env, #(#args),*) -> R
    }
}
