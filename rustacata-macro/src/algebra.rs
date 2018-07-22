use std::iter::{Iterator, IntoIterator, FromIterator};

use proc_macro2::{Span, TokenStream};

use syn::parse;
use syn::{Ident, Expr, FnArg, Fields, Field, Variant, Type, Pat, Arm, GenericParam, WherePredicate, ItemFn, FieldValue};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::{Args, Datatype};

pub struct Env(());

impl Env {
    pub fn new() -> Self {
        Env(())
    }

    pub fn datatype_ty(&self, dt: &Datatype) -> Type {
        let dt_ty = dt.ty();
        parse_quote! { &'b #dt_ty }
    }

    pub fn field_arg_ty(&self, field: &Field) -> Type {
        let arg_ty = &field.ty;
        parse_quote! { &'b #arg_ty }
    }

    pub fn initializer_arg_ty(&self, field: &Field) -> Type {
        let arg_ty = &field.ty;
        parse_quote! { &#arg_ty }
    }

    pub fn argument_pat(&self, ident: &Ident) -> Pat {
        parse_quote! { ref #ident }
    }

    pub fn default_result_ty(&self) -> Type {
        parse_quote! { B }
    }

    pub fn generics(&self, dt: &Datatype) -> Punctuated<GenericParam, Comma> {
        parse_quote! { 'b }
    }

    pub fn generics_bounds(&self, dt: &Datatype) -> Punctuated<WherePredicate, Comma> {
        Punctuated::new()
    }
}

pub trait Algebra {

    fn trait_name(&self, dt: &Datatype) -> Ident;

    fn struct_name(&self, dt: &Datatype) -> Ident;

    fn result_type(&self, dt: &Datatype) -> Type;

    fn generics(&self, dt: &Datatype) -> Punctuated<GenericParam, Comma>;

    fn generics_bounds(&self, dt: &Datatype) -> Punctuated<WherePredicate, Comma>;

    fn field_name(&self, ident: &Ident) -> Ident;

    fn setter_name(&self, ident: &Ident) -> Ident;

    fn initializer_body(&self, ident: &Ident, args: &Vec<FnArg>) -> Expr;
}

//pub fn generate<Alg: Algebra>(env: &Env, dt: &Datatype) -> TokenStream {
//    let dt_ty = env.datatype_ty(dt);
//    let dt_arg : Ident = parse_quote!( x );
//
//    let alg_trait = Alg::trait_name(env, dt);
//    let alg_struct = Alg::struct_name(env, dt);
//
//    let alg_result_ty = Alg::result_type(env, dt);
//
//    let env_generics = env.generics(dt);
//    let env_generics_bounds = env.generics_bounds(dt);
//
//    let alg_generics = Alg::generics(env, dt);
//    let alg_generics_bounds = Alg::generics_bounds(env, dt);
//
////    let all_generics_bounds : Punctuated<WherePredicate, Comma> =
////        env.generics_bounds(dt).into_iter().chain(alg_generics_bounds).collect();
//
//    let alg_fields = apply_to_variants(env, dt, field::<Alg>);
//    let alg_setters = apply_to_variants(env, dt, setter::<Alg>);
//    let alg_inits = apply_to_variants(env, dt, initializer::<Alg>);
//    let match_arms = apply_to_variants(env, dt, match_arm::<Alg>);
//
//    quote! {
//        struct #alg_struct<'a, #alg_generics> {
//            #(#alg_fields),*
//        }
////
//        impl<'a, #alg_generics> #alg_struct<'a, #alg_generics> {
//            #(#alg_setters)*
//        }
////
//        impl<'a, #env_generics, #alg_generics> Transformer<#dt_ty, #alg_result_ty>
//        for #alg_struct<'a, #alg_generics> {
////        where #env_generics, alg_generics_bounds {
//            fn transform(&self, #dt_arg: #dt_ty) -> #alg_result_ty {
//                match *#dt_arg {
//                    #(#match_arms),*
//                }
//            }
//        }
////
//        impl <#env_generics, #alg_generics> #alg_trait<#alg_generics> for #dt_ty {
//            type Tr = #alg_struct<'static, #alg_generics>;
//
//            fn transformer() -> Self::Tr {
//                #alg_struct {
//                    #(#alg_inits),*
//                }
//            }
//        }
//    }
//}
//
//fn apply_to_variants<'a, R: 'a, F: 'a>(env: &'a Env, dt: &'a Datatype, f: F) -> impl 'a + Iterator<Item = R>
//where
////    R: 'a,
//    F: Fn(&'a Env, &'a Datatype, &'a Ident, &'a Fields) -> R,
//{
//    match dt {
//        Datatype::Enum(ref item) => item.variants.iter().map(move |variant| {
//            f(env, dt, &variant.ident, &variant.fields)
//        }),
//        _ => unimplemented!()
//    }
//}
//
//fn match_arm<Alg: Algebra>(env: &Env, dt: &Datatype, ident: &Ident, fields: &Fields) -> Arm {
//    let field_name = Alg::field_name(env, ident);
//    let pat = match_pat(env, dt, ident, fields);
//    let args = arg_names(env, dt, ident, fields);
//
//    parse_quote! {
//        #pat => (self.#field_name)(self, #(#args),*)
//    }
//}
//
////fn field<'a, 'b, Alg: Algebra>(env: &'a Env, dt: &'b Datatype, ident: &'b Ident, fields: &'b Fields) -> Field {
//fn field<'a, 'b, Alg: Algebra>(env: &'a Env, dt: &'b Datatype, ident: &'b Ident, fields: &'b Fields) -> TokenStream {
//    let field_name = Alg::field_name(env, ident);
//    let field_fn_ty = field_fn_ty::<Alg>(env, dt, fields);
//
//    quote! {
//        #field_name : Box<'a + #field_fn_ty>
//    }
//}
//
//fn setter<'a, 'b, Alg: Algebra>(env: &'a Env, dt: &'b Datatype, ident: &'b Ident, fields: &'b Fields) -> ItemFn {
//    let struct_name = Alg::struct_name(env, dt);
//    let field_name = Alg::field_name(env, ident);
//    let setter_name = Alg::setter_name(env, ident);
//
//    let field_fn_ty = field_fn_ty::<Alg>(env, dt, fields);
//
//    parse_quote! {
//        fn #setter_name<'c: 'a, F>(self, f: F) -> Self
//        where
//            F: 'c + #field_fn_ty
//        {
//            #struct_name { #field_name: Box::new(f), ..self }
//        }
//    }
//}
//
//fn initializer<'a, 'b, Alg: Algebra>(env: &'a Env, dt: &'b Datatype, ident: &'b Ident, fields: &'b Fields) -> FieldValue {
//    let args = initializer_arguments(env, dt, ident, fields);
//
//    let field = Alg::field_name(env, ident);
////    let body: Expr = parse_quote! { unimplemented!() };
//    let body = Alg::initializer_body(env, ident, &args);
//
//    parse_quote! {
//        #field: Box::new(|tr, #(#args),*| #body)
//    }
//}
//
//fn initializer_arguments<'a>(env: &'a Env, dt: &'a Datatype, ident: &'a Ident, fields: &'a Fields) -> Vec<FnArg> {
//    arg_names(env, dt, ident, fields)
//        .zip(fields.iter().map(move |field| env.initializer_arg_ty(field)))
//        .map(|(arg, ty)| parse_quote! { #arg: #ty })
//        .collect()
//}
//
//fn match_pat(env: &Env, dt: &Datatype, ident: &Ident, fields: &Fields) -> Pat {
//    let dt_ident = dt.ident();
//    let args = arg_names(env, dt, ident, fields)
//        .map(|arg| env.argument_pat(&arg));
//
//    match fields {
//        Fields::Unnamed(_) => {
//            parse_quote! {
//                #dt_ident::#ident(#(#args),*)
//            }
//        },
//        _ => unimplemented!(),
//    }
//}
//
//fn field_fn_ty<Alg: Algebra>(env: &Env, dt: &Datatype, fields: &Fields) -> Type {
//    let dt_ty = env.datatype_ty(dt);
//    let r_ty = Alg::result_type(env, dt);
//
//    let dt_tr = quote! { &Transformer<#dt_ty, #r_ty> };
//    let args = fields.iter().map(|field| env.field_arg_ty(field));
//
//    parse_quote! {
//        for<'b> Fn(#dt_tr, #(#args),*) -> #r_ty
//    }
//}
//
//fn arg_names<'a>(env: &'a Env, dt: &'a Datatype, ident: &'a Ident, fields: &'a Fields) -> impl 'a + Iterator<Item = Ident> {
//    let mut cnt = 0;
//
//    let mut gen = move || {
//        cnt = cnt + 1;
//        Ident::new(&format!("x{}", cnt - 1), Span::call_site())
//    };
//
//    fields.iter().map(move |field| {
//        if let Some(ref ident) = field.ident { ident.clone() } else { gen() }
//    })
//}