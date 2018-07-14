
use proc_macro2::{Span, TokenStream};

use syn::parse;
use syn::{Ident, Expr, FnArg, Fields, Field, Variant, Type, Pat, Arm, GenericParam, WherePredicate, ItemFn, FieldValue};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use algebra::{Env, Algebra};
use input::{Datatype};

pub struct Foldable;

impl Algebra for Foldable {
    fn trait_name(_env: &Env, _dt: &Datatype) -> Ident {
        Ident::new("Foldable", Span::call_site())
    }

    fn struct_name(_env: &Env, dt: &Datatype) -> Ident {
        Ident::new(&format!("{}Fold", dt.ident()), Span::call_site())
    }

    fn result_type(env: &Env, dt: &Datatype) -> Type {
        env.default_result_ty()
    }

    fn generics(env: &Env, dt: &Datatype) -> Punctuated<GenericParam, Comma> {
        let r_ty = env.default_result_ty();
        parse_quote! { #r_ty }
    }

    fn generics_bounds(env: &Env, dt: &Datatype) -> Punctuated<WherePredicate, Comma> {
        Punctuated::new()
    }

    fn field_name(_env: &Env, ident: &Ident) -> Ident {
        Ident::new(&format!("fold_{}", ident).to_lowercase(), Span::call_site())
    }

    fn setter_name(_env: &Env, ident: &Ident) -> Ident {
        Ident::new(&format!("with_fold_{}", ident).to_lowercase(), Span::call_site())
    }

    fn initializer_body(_env: &Env, ident: &Ident, _args: &Vec<FnArg>) -> Expr {
        parse_quote! { unimplemented!() }
    }
}