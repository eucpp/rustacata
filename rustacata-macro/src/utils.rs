use std::collections::HashSet;

use syn::{Ident, Field, Pat, FnArg, GenericParam};
use proc_macro2::{Span};

use traverse::{TraversePolicy};

pub struct ArgGen {
    cnt: u32,
}

impl ArgGen {
    pub fn new() -> Self {
        ArgGen { cnt: 0 }
    }

    pub fn iter_fields<'a, I, R: 'a, F: 'a>(&mut self, it: I, mut f: F) -> Vec<R>
    where
        I: IntoIterator<Item = &'a Field>,
        F: FnMut(&mut Self, &Field) -> R,
    {
        it.into_iter()
            .map(|field| f(self, field))
            .collect()
    }

    pub fn ident(&mut self, field: &Field) -> Ident {
        if let Some(ref ident) = field.ident {
            ident.clone()
        } else {
            let cnt = self.cnt;
            self.cnt = self.cnt + 1;
            Ident::new(&format!("x{}", cnt), Span::call_site())
        }
    }

    pub fn pat<Trv: TraversePolicy>(&mut self, trv: &Trv, field: &Field) -> Pat {
        trv.datatype_field_pat(&self.ident(field))
    }

    pub fn fn_arg<Trv: TraversePolicy>(&mut self, trv: &Trv, field: &Field) -> FnArg {
        let ident = self.ident(field);
        let ty = trv.initializer_arg_ty(field);
        parse_quote! { #ident: #ty }
    }

}

pub struct IdentMangler<'a> {
    used: HashSet<Ident>,
    renamer: Box<'a + FnMut(String) -> String>,
}

impl<'a> IdentMangler<'a> {
    pub fn new<F: 'a + FnMut(String) -> String>(renamer: F) -> Self {
        IdentMangler {
            used: HashSet::new(),
            renamer: Box::new(renamer),
        }
    }

    pub fn reserve(&mut self, ident: &Ident) {
        assert!(!self.used.contains(ident));
        self.used.insert(ident.clone());
    }

    pub fn mangle(&mut self, ident: &Ident)-> Ident {
        let mangled = Ident::new(
            &(self.renamer)(ident.to_string()),
            ident.span()
        );
        assert!(!self.used.contains(&mangled));
        self.used.insert(mangled.clone());
        mangled
    }
}