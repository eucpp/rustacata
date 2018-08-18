
use proc_macro2::{Span, TokenStream};

use syn::parse;
use syn::{Ident, Expr, FnArg, Fields, Field, Variant, Type, Pat, Arm, GenericParam, WherePredicate, ItemFn, FieldValue};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use algebra::{Algebra};
use input::{Datatype};

pub struct Foldable(());

impl Foldable {
    pub fn new() -> Self {
        Foldable(())
    }

    fn result_type_param_ident(&self) {
        Ident::new("B", Span::call_site())
    }

    fn
}

impl Algebra for Foldable {

    fn trait_ident(&self, _dt: &Datatype) -> Ident {
        Ident::new("Foldable", Span::call_site())
    }

    fn struct_ident(&self, dt: &Datatype) -> Ident {
        Ident::new(&format!("{}Fold", dt.ident()), Span::call_site())
    }

    fn result_type(&self, dt: &Datatype) -> Type {
        parse_quote! { B }
    }

    fn generics(&self, dt: &Datatype) -> Punctuated<GenericParam, Comma> {
        let mut res = Punctuated::<GenericParam, Comma>::new();
        let mut gen = GenericParamGen::new();

        res.push(gen.generic_param(self.result_type_param()));

        for param in dt.type_params() {
            res.push(GenericParam::Type(param));
            res.push(gen.generic_param(self.type_param(param)));
        }

        res
    }

    fn generics_bounds(&self, dt: &Datatype) -> Punctuated<WherePredicate, Comma> {
        Punctuated::new()
    }

    fn field_ident(&self, ident: &Ident) -> Ident {
        Ident::new(&format!("fold_{}", ident).to_lowercase(), Span::call_site())
    }

    fn setter_ident(&self, ident: &Ident) -> Ident {
        Ident::new(&format!("with_fold_{}", ident).to_lowercase(), Span::call_site())
    }

    fn initializer_body(&self, ident: &Ident, _args: &Vec<FnArg>) -> Expr {
        parse_quote! { unimplemented!() }
    }
}