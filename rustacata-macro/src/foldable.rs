use proc_macro2::{Span, TokenStream};

use syn::parse;
use syn::{Ident, Expr, FnArg, Fields, Field, Variant, Type, Pat, Arm, GenericParam, WherePredicate, ItemFn, FieldValue, TypeParam};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use algebra::{Algebra};
use input::{Datatype};
use utils::{IdentMangler};

pub struct Foldable(());

impl Foldable {
    pub fn new() -> Self {
        Foldable(())
    }

    fn rename_ty(s: &str) -> String {
        format!{"{}__", s}
    }

    fn result_type_ident(&self) -> Ident {
        Ident::new(&Self::rename_ty("R"), Span::call_site())
    }

    fn type_param_mangler(&self) -> IdentMangler<'static> {
        let mut mangler = IdentMangler::new(|s| Self::rename_ty(&s));
        mangler.reserve(&self.result_type_ident());
        mangler
    }
}

impl Algebra for Foldable {

    fn trait_ident(&self, dt: &Datatype) -> Ident {
        let n = dt.type_params().count();
        if n == 0 {
            Ident::new("Foldable", Span::call_site())
        } else {
            Ident::new(&format!("Foldable{}", n), Span::call_site())
        }
    }

    fn struct_ident(&self, dt: &Datatype) -> Ident {
        Ident::new(&format!("{}Fold", dt.ident()), Span::call_site())
    }

    fn result_type(&self, dt: &Datatype) -> Type {
        let ident = self.result_type_ident();
        parse_quote! { #ident }
    }

    fn generics(&self, dt: &Datatype) -> Punctuated<GenericParam, Comma> {
        let mut res = Punctuated::<GenericParam, Comma>::new();

        let mut mangler = self.type_param_mangler();

        let make_generic =
            |ident: Ident| GenericParam::Type(TypeParam::from(ident));

        res.push(make_generic(self.result_type_ident()));

        for param in dt.type_params() {
            mangler.reserve(&param.ident);
            res.push(GenericParam::Type(param.clone()));
            res.push(make_generic(mangler.mangle(&param.ident)));
        }

        res
    }

    fn generics_bounds(&self, dt: &Datatype) -> Punctuated<WherePredicate, Comma> {
        Punctuated::new()
//        dt.type_params_bounds()
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