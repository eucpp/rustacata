
use syn::{parse, Ident, Expr, Variant, Type, GenericParam, WherePredicate};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use algebra::{Env, Algebra};

struct Foldable;

impl Algebra for Foldable {
    fn trait_name() -> Ident {
        Ident::new("Foldable", Span::call_site())
    }

    fn struct_name(_env: &Env, dt: &Datatype) -> Ident {
        Ident::new(&format!("{}Fold", *ident), Span::call_site())
    }

    fn generics(env: &Env, dt: &Datatype) -> Punctuated<GenericParam, Comma> {
        let r_ty = env.default_result_ty();
        parse( quote! { #r_ty } ).unwrap()
    }

    fn generics_bounds(env: &Env, dt: &Datatype) -> Punctuated<WherePredicate, Comma> {
        Punctuated::new()
    }

    fn variant_field_name(_env: &Env, variant: &Variant) -> Ident {
        Ident::new(&format!("fold_{}", variant.ident).to_lowercase(), Span::call_site())
    }

    fn variant_setter_name(_env: &Env, variant: &Variant) -> Ident {
        Ident::new(&format!("with_fold_{}", variant.ident).to_lowercase(), Span::call_site())
    }

    fn variant_field_default(_env: &Env, variant: &Variant) -> Expr {
        quote! { unimplemented!() }
    }
}