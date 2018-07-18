
use algebra::Algebra;
use traverse::TraversePolicy;

struct Catamorphism<Alg: Algebra, Trv: TraversePolicy> {
    dt: Datatype,
    alg: Alg,
    trv: Trv,
}

impl<Alg: Algebra, Trv: TraversePolicy> Catamorphism<Alg, Trv> {

    pub fn codegen(&self) -> TokenStream {
        let dt_ty = self.datatype_ty();
        let dt_arg = self.datatype_arg_name();
        let dt_tr_ty = self.transformer_ty();

        let alg_trait = self.alg_trait_name();
        let alg_struct = self.alg_struct_name();
        let alg_result_ty = self.alg_result_type();

        let alg_generics = self.alg_generics();
        let all_generics = self.all_generics();

        let alg_generics_bounds = self.alg_generics_bounds();
        let all_generics_bounds = self.all_generics_bounds();

        let alg_fields = self.alg_fields();
        let alg_setters = self.alg_setters();
        let alg_inits = self.alg_inits();
        let alg_match_arms = self.alg_match_arms();

        quote! {
            struct #alg_struct<'a, #alg_generics> #alg_fields

            impl<'a, #alg_generics> #alg_struct<'a, #alg_generics> {
                #(#alg_setters)*
            }

            impl<'a, #all_generics> #dt_tr_ty
            for #alg_struct<'a, #alg_generics>
            where #all_generics_bounds {
                fn transform(&self, #dt_arg: #dt_ty) -> #alg_result_ty {
                    match *#dt_arg {
                        #(#match_arms),*
                    }
                }
            }

            impl <#all_generics> #alg_trait<#alg_generics> for #dt_ty
            where #all_generics_bounds {
                type Tr = #alg_struct<'static, #alg_generics>;

                fn transformer() -> Self::Tr {
                    #alg_struct {
                        #(#alg_inits),*
                    }
                }
            }
        }
    }

    fn alg_trait_name(&self) -> Ident {
        self.alg.trait_name(self.dt)
    }

    fn alg_struct_name(&self) -> Ident {
        self.alg.struct_name(self.dt)
    }

    fn alg_result_type(&self) -> Type {
        self.alg.result_type(self.dt)
    }

    fn alg_generics(&self) -> Punctuated<GenericParam, Comma> {
        self.alg.generics(self.dt)
    }

    fn all_generics(&self) -> Punctuated<GenericParam, Comma> {
        let lifetimes = self.trv.lifetimes(self.dt);
        let alg_generics = self.alg_generics();
        parse_quote! { #(#lifetimes),* #(#alg_generics) }
    }

    fn alg_generics_bounds(&self) -> Punctuated<WherePredicate, Comma> {
        self.alg.generics_bounds(self.dt)
    }

    fn all_generics_bounds(&self) -> Punctuated<WherePredicate, Comma> {
        let lifetimes_bounds = self.trv.lifetimes_bounds(self.dt);
        let alg_generics_bounds = self.alg.alg_generics_bounds();
        parse_quote! { #(#lifetimes_bounds),* #(#alg_generics_bounds),* }
    }

    fn alg_fields(&self) -> Fields {
        let fields = self.apply_to_variants(self.alg_field)
            .map(|(ident, ty)| {
                quote! { #field: #ty }
            });

        parse_quote! { { #(#fields),* } }
    }

    fn alg_field(&self, ident: &Ident, fields: &Fields) -> (Ident, Type) {
        let field_fn_ty = self.alg_field_fn_ty(ident, fields);

        (self.alg_field_name(ident), parse_quote! { Box<'a + #field_fn_ty> })
    }

    fn alg_field_name(&self, ident: &Ident) -> Ident {
        self.alg.field_name(ident)
    }

    fn alg_field_fn_ty(&self, ident: &Ident, fields: &Fields) -> Type {
        let dt_ty = self.datatype_ty();
        let r_ty = self.alg_result_ty();
        let dt_tr_ty = self.datatype_transformer_ty();
        let args = fields.iter().map(|field| self.fn_arg_ty(field));

        self.trv.fn_type( parse_quote! { Fn(&#dt_tr_ty, #(#args),*) -> #r_ty } )
    }

    fn alg_setter_name(&self, ident: &Ident) {
        self.alg.setter_name(ident)
    }

    fn alg_initializer(&self, ident: &Ident, fields: &Fields) -> FieldValue {
        let alg_field_name = self.alg_field_name();
        let args = self.alg_initializer_args(ident, fields);
        let body = self.alg_initializer_body(ident, &args);

        parse_quote! {
            #field: Box::new(|tr, #(#args),*| #body)
        }
    }

    fn alg_initializer_args(&self, ident: &Ident, fields: &Fields) -> Vec<FnArg> {

    }

    fn alg_initializer_body(&self, ident: &Ident, args: &Vec<FnArg>) -> Expr {
        self.alg.initializer_body(ident, args)
    }

    fn apply_to_variants<'a, R: 'a, F: 'a>(&self, f: F) -> impl 'a + Iterator<Item = R>
        where
            F: Fn(&'a Self, &'a Ident, &'a Fields) -> R,
    {
        match self.dt {
            Datatype::Enum(ref item) => item.variants.iter().map(move |variant| {
                f(self, &variant.ident, &variant.fields)
            }),
            _ => unimplemented!()
        }
    }

}