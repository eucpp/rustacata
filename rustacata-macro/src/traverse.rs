
use syn::{Ident, Type, Pat, Field, Lifetime, PredicateLifetime, TypeParam};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::Datatype;

pub trait TraversePolicy {
    fn datatype_ty(&self, dt: &Datatype) -> Type;

    fn datatype_field_pat(&self, ident: &Ident) -> Pat;

    fn fn_type(&self, ty: &Type) -> Type;

    fn fn_arg_type(&self, field: &Field) -> Type;

    fn type_param_type(&self, ty_param: &TypeParam) -> Type;

    fn initializer_arg_ty(&self, field: &Field) -> Type;

    fn lifetimes(&self, dt: &Datatype) -> Punctuated<Lifetime, Comma>;

    fn lifetimes_bounds(&self, dt: &Datatype) -> Punctuated<PredicateLifetime, Comma>;
}

pub struct BorrowTraverse(());

impl BorrowTraverse {
    pub fn new() -> Self {
        BorrowTraverse(())
    }
}

impl TraversePolicy for BorrowTraverse {
    fn datatype_ty(&self, dt: &Datatype) -> Type {
        let dt_ty = dt.ty();
        parse_quote! { &'b #dt_ty }
    }

    fn datatype_field_pat(&self, ident: &Ident) -> Pat {
        parse_quote! { ref #ident }
    }

    fn fn_type(&self, ty: &Type) -> Type {
        parse_quote! { for<'b> #ty }
    }

    fn fn_arg_type(&self, field: &Field) -> Type {
        let arg_ty = &field.ty;
        parse_quote! { &'b #arg_ty }
    }

    fn type_param_type(&self, ty_param: &TypeParam) -> Type {
        let ident = &ty_param.ident;
        parse_quote! { &'b #ident }
    }

    fn initializer_arg_ty(&self, field: &Field) -> Type {
        let arg_ty = &field.ty;
        parse_quote! { &#arg_ty }
    }

    fn lifetimes(&self, dt: &Datatype) -> Punctuated<Lifetime, Comma> {
        parse_quote! { 'b }
    }

    fn lifetimes_bounds(&self, dt: &Datatype) -> Punctuated<PredicateLifetime, Comma> {
//        parse_quote! { }
        Punctuated::new()
    }
}