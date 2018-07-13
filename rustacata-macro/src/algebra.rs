use std::iter::{IntoIterator};

use proc_macro2::{Span, TokenStream};

use syn::{parse, Ident, Expr, Variant, Type, GenericParam, WherePredicate};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::{Args, Datatype};

struct Env {

}

impl Env {
    fn default_result_ty(&self) -> Type {
        r_ty = Ident::new("R", Span::call_site());
        parse(quote! { #r_ty })
    }
}

pub trait Algebra {

    fn trait_name() -> Ident;

    fn struct_name(env: &Env, dt: &Datatype) -> Ident;

    fn result_type(env: &Env, dt: &Datatype) -> Type {
        env.default_result_ty()
    }

    fn generics(dt: &Datatype) -> Punctuated<GenericParam, Comma>;

    fn generics_bounds(dt: &Datatype) -> Punctuated<WherePredicate, Comma>;

    fn variant_field_name(variant: &Variant) -> Ident;

    fn variant_setter_name(variant: &Variant) -> Ident;

    fn variant_field_default(variant: &Variant) -> Expr;
}

fn generate<A: Algebra>(args: &Args, dt: &Datatype, alg: &A) -> TokenStream {
    let self_ty = dt.ty();

    let alg_name = alg.name(dt);

    let alg_result_ty = alg.result_ty(dt);
    let alg_trait_name = alg.trait_name();

    let alg_generics = alg.generics(dt);
    let alg_generics_bounds = alg.generics_bounds(dt);

    let alg_setters = match dt {
        Data::Enum(item) => item.variants.iter().map(|variant| {
            gen_variant_setter(dt, &variant, alg)
        }),
    };

    let alg_inits = match dt {
        Data::Enum(item) => item.variants.iter().map(|variant| {
            gen_variant_init(dt, &variant, alg)
        }),
    };

    quote! {
//        struct #alg_name<'a, #alg_generics>(#alg_tbl<'a, #alg_result_ty>);

        struct #alg_name<'a, #alg_generics> {
            #(#fields),*
        }

        impl #alg_name<'a, #alg_generics> {
            #(#alg_setters),*
        }

        impl<'a, 'b, #alg_generics> Transformer<&'b #self_ty, #alg_result_ty>
        for #alg_name<'a, #alg_generics>
        where #alg_generics_bounds {

            fn transform(&self, x: &'b #self_ty) -> #alg_result_ty {
                self.0.transform(x)
            }
        }

        impl <#alg_generics_bounds> #alg_trait_name<#alg_generics> for #self_ty {
            type Tr = #alg_name<'static, #alg_generics>;

            fn transformer() -> Self::Tr {
                #alg_name {
                    #(#alg_inits),*
                }
            }
        }
    }
}

fn gen_variant_setter<A: Algebra>(dt: &Datatype, variant: &Variant, alg: &A) -> TokenStream {
    let alg_name = alg.name(dt.ident());
    let field_name = alg.variant_field_name(&variant.ident);
    let setter_name = alg.variant_setter_name(&variant.ident);

    let fn_ty = variant_fn_type(dt, variant);

    quote! {
        fn #setter_name <'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + #fn_ty
        {
            #alg_name { #field_name: Box::new(f), ..self }
        }
    }
}

fn gen_variant_init<A: Algebra>(dt: &Datatype, variant: &Variant, alg: &A) -> TokenStream {
    let field_name = alg.variant_field_name(variant);
    let body = alg.variant_field_default(variant);

    let mut cnt = 0;
    let args = variant.fields.iter().map(|field| {
        let arg_name = Ident::new(&format!("x{}", cnt), Span::call_site());
        let arg_ty = field.ty;

        cnt = cnt + 1;

        quote! {
            #arg_name: &'b #arg_ty
        }
    });

    quote! {
        #field_name: Box::new(|tr, #(#args),*| #body)
    }
}

fn variant_fn_type(dt: &Datatype, variant: &Variant) -> TokenStream {
    let dt_ty = dt.ty();
    let dt_tr = quote! { Transformer<& 'b #dt_ty, R, Env> };

    let args = variant.fields.iter().map(|field| {
        let field_ty = &field.ty;
        quote! { & 'b #field_ty }
    });

    quote! {
        for<'b> Fn(#dt_tr, Env, #(#args),*) -> R
    }
}