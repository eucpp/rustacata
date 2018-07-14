use std::iter::{IntoIterator};

use proc_macro2::{Span, TokenStream};

use syn::{parse, Ident, Expr, FnArg, Fields, Field, Variant, Type, GenericParam, WherePredicate};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::{Args, Datatype};

struct Env {

}

impl Env {
    pub fn datatype_ty(&self, dt: &Datatype) -> Type {
        let dt_ty = dt.ty();
        parse_quote! { &'b #self_ty }
    }

    pub fn argument_ty(&self, field: &Field) -> Type {
        let arg_ty = field.ty();
        parse_quote! { &'b #arg_ty }
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

    fn trait_name(env: &Env, dt: &Datatype) -> Ident;

    fn struct_name(env: &Env, dt: &Datatype) -> Ident;

    fn result_type(env: &Env, dt: &Datatype) -> Type;

    fn generics(env: &Env, dt: &Datatype) -> Punctuated<GenericParam, Comma>;

    fn generics_bounds(env: &Env, dt: &Datatype) -> Punctuated<WherePredicate, Comma>;

    fn field_name(env: &Env, ident: &Ident) -> Ident;

    fn setter_name(env: &Env, ident: &Ident) -> Ident;

    fn initializer_body(env: &Env, ident: &Ident, args: &Vec<FnArg>) -> Expr;
}

fn generate<Alg: Algebra>(env: &Env, dt: &Datatype) -> TokenStream {
    let dt_ty = env.datatype_ty(dt);

    let alg_trait_name = Alg::trait_name(env, dt);
    let alg_struct_name = Alg::struct_name(env, dt);

    let alg_result_ty = Alg::result_type(env, dt);

    let alg_generics = Alg::generics(env, dt);
    let alg_generics_bounds = Alg::generics_bounds(env, dt);

    let all_generics = env.generics(dt).extend(alg_generics);
    let all_generics_bounds = env.generics_bounds(dt).extend(alg_generics_bounds);

    let alg_fields = apply_to_variants(env, dt, field::<Alg>);
    let alg_setters = apply_to_variants(env, dt, setter::<Alg>);
    let alg_inits = apply_to_variants(env, dt, initializer::<Alg>);
    let match_arms = apply_to_variants(env, dt, match_arm::<Alg>);

    quote! {
        struct #alg_struct<'a, #alg_generics> {
            #(#alg_fields),*
        }

        impl #alg_struct<'a, #alg_generics> {
            #(#alg_setters),*
        }

        impl<'a, #all_generics> Transformer<#dt_ty, #alg_result_ty>
        for #alg_struct<'a, #alg_generics>
        where #all_generics_bounds {
            fn transform(&self, #dt_arg: #dt_ty) -> #alg_result_ty {
                #(#match_arms),*
            }
        }

        impl <#alg_generics_bounds> #alg_trait<#alg_generics> for #dt_ty {
            type Tr = #alg_struct<'static, #alg_generics>;

            fn transformer() -> Self::Tr {
                #alg_struct {
                    #(#alg_inits),*
                }
            }
        }
    }
}

fn apply_to_variants<F, R>(env: &Env, dt: &Datatype, f: F) -> Iter<Item = R>
    where
        F: Fn(&Env, &Datatype, &Ident, &Fields) -> R
{
    match dt {
        Data::Enum(item) => item.variants.iter().map(|variant| {
            f(env, dt, &variant.ident, &variant.fields)
        }),
        _ => unimplemented!()
    }
}

fn match_arm<Alg: Algebra>(env: &Env, dt: &Datatype, ident: Ident, fields: &Fields) -> Arm {
    let field_name = Alg::field_name(env, ident);
    let pat = match_pat(env, dt, ident, fields);
    let args = arg_names(env, dt, ident, fields);

    parse_quote! {
        #pat => (self.#field_name)(self, #args)
    }
}

fn field<Alg: Algebra>(env: &Env, dt: Datatype, ident: Ident, fields: &Fields) -> Field {
    let field_name = Alg::field_name(env, ident);
    let field_fn_ty = field_fn_ty(env, dt, fields);

    parse_quote! {
        #field_name : Box<'a + #field_fn_ty>
    }
}

fn setter<Alg: Algebra>(env: &Env, dt: &Datatype, ident: &Ident, fields: &Fields) -> ItemFn {
    let struct_name = Alg::struct_name(env, dt.ident());
    let field_name = Alg::field_name(env, ident);
    let setter_name = Alg::setter_name(env, ident);

    let field_fn_ty = field_fn_ty::<Alg>(env, dt, fields);

    parse_quote! {
        fn #setter_name <'c: 'a, F>(self, f: F) -> Self
        where
            F: 'c + #field_fn_ty
        {
            #struct_name { #field_name: Box::new(f), ..self }
        }
    }
}

fn initializer<Alg: Algebra>(env: &Env, dt: &Datatype, ident: &Ident, fields: &Fields) -> FieldValue {
    let args = initializer_arguments(env, dt, fields);

    let field = Alg::field_name(env, ident);
    let body = Alg::initializer_body(env, ident, Vec::from_iter(args));

    parse_quote! {
        #field: Box::new(|tr, #(#args),*| #body)
    }
}

fn initializer_arguments(env: &Env, dt: &Datatype, fields: &Fields) -> Iter<Item = FnArg> {
    arg_names(env, dt, ident, fields)
        .zip(fields.iter().map(|field| env.argument_ty(field)))
        .map(|arg, ty| parse_quote! { #arg: #ty })
}

fn match_pat(env: &Env, dt: &Datatype, ident: &Ident, fields: &Fields) -> Pat {
    let args = arg_names(env, dt, ident, fields)
        .map(|arg| env.argument_pat(arg));

    match fields {
        Fields::Unnamed(_) => {
            parse_quote! {
                #ident(#(#args),*)
            }
        },
        _ => unimpelmented!(),
    }
}

fn field_fn_ty<Alg: Algebra>(env: &Env, dt: &Datatype, fields: &Fields) -> Type {
    let dt_ty = env.datatype_ty(dt);
    let r_ty = Alg::result_type(env, dt);

    let dt_tr = quote! { Transformer<#dt_ty, #r_ty> };
    let args = fields.iter().map(|field| env.argument_ty(field));

    parse_quote! {
        for<'b> Fn(#dt_tr, #(#args),*) -> #r_ty
    }
}

fn arg_names(env: &Env, dt: &Datatype, ident: Ident, fields: &Fields) -> Iter<Item = Ident> {
    let mut cnt = 0;

    let gen = || {
        cnt = cnt + 1;
        Ident::new(&format!("x{}", cnt - 1), Span::call_site())
    };

    fields.iter().map(|field| {
        if let Some(ident) = field.ident { ident } else { gen() };
    })
}