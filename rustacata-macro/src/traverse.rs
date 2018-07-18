
use syn::{Ident, Type, Pat, Field, Lifetime, PredicateLifetime};

pub trait TraversePolicy {
    fn datatype_ty(&self, dt: &Datatype) -> Type;

    fn datatype_field_pat(&self, ident: &Ident) -> Pat;

    fn fn_type(&self, ty: &Type) -> Type;

    fn fn_arg_type(&self, field: &Field) -> Type;

    fn initializer_arg_ty(&self, field: &Field) -> Type;

    fn lifetimes(&self, dt: &Datatype) -> Vec<Lifetime>;

    fn lifetimes_bounds(&self, dt: &Datatype) -> Vec<PredicateLifetime>;
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

    fn initializer_arg_ty(&self, field: &Field) -> Type {
        let arg_ty = &field.ty;
        parse_quote! { &#arg_ty }
    }

    fn lifetimes(&self, dt: &Datatype) -> Vec<Lifetime> {
        vec![
            parse_quote! { 'b },
        ]
    }

    fn lifetimes_bounds(&self, dt: &Datatype) -> Vec<PredicateLifetime> {
        vec![]
    }
}