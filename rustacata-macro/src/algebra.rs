use std::iter::{Iterator, IntoIterator, FromIterator};

use proc_macro2::{Span, TokenStream};

use syn::parse;
use syn::{Ident, Expr, FnArg, Type, GenericParam, WherePredicate};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::{Args, Datatype};

pub trait Algebra {

    fn trait_ident(&self, dt: &Datatype) -> Ident;

    fn struct_ident(&self, dt: &Datatype) -> Ident;

    fn result_type(&self, dt: &Datatype) -> Type;

    fn generics(&self, dt: &Datatype) -> Punctuated<GenericParam, Comma>;

    fn generics_bounds(&self, dt: &Datatype) -> Punctuated<WherePredicate, Comma>;

    fn field_ident(&self, ident: &Ident) -> Ident;

    fn setter_ident(&self, ident: &Ident) -> Ident;

    fn initializer_body(&self, ident: &Ident, args: &Vec<FnArg>) -> Expr;
}