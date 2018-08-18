use std::collections::HashMap;

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
//        let ty = &field.ty;
        parse_quote! { #ident: #ty }
    }

}

pub struct IdentMangler {
    used : HashSet<Ident>
}

impl IdentMangler {
    pub fn new() -> Self {
        IdentMangler {
            used: HashSet::new(),
        }
    }

    pub fn mangle(&mut self, mut ident: Ident)-> Ident {
        let mut i = 0;
        while self.used.contains(ident) {
            i = i + 1;
            ident = Ident::new(
                &format!{"{}_{}", ident, i},
                ident.span()
            )
        }
        self.used.insert(ident);
        ident
    }
}

pub struct GenericParamGen {
    map: HashMap<Ident, GenericParam>
}

impl GenericParamGen {

    pub fn generic_param(&mut self, hint: &GenericParam) -> GenericParam {
        let ident =
        if self.map.contains_key() {

        }

        match *hint {

            GenericParam::Type(ref type_param) => {
                if self.map.contains_key(type_param.ident) {
                    let mangled = self.mangle_type_param(type_param);
                    self.map.insert(mangled.ident.clone(), mangled.clone());
                    GenericParam::Type(mangled)
                } else {
                    self.map.insert(type_param.ident.clone(), type_param.clone());
                    hint.clone()
                }
            },

            GenericParam::Lifetime(ref lifetime_def) => {

            }
        }

        if self.map.contains_key(self.generic) {

        }
    }

}