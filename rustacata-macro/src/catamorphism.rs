
use proc_macro2::{Span, TokenStream};

use syn::{Ident, Expr, FnArg, Fields, FieldsNamed, Field, Variant, Type, Pat, Arm, GenericParam, WherePredicate, ItemFn, FieldValue, TypeParam};
use syn::token::{Comma};
use syn::punctuated::{Punctuated};

use input::{Datatype, Args};
use algebra::Algebra;
use traverse::TraversePolicy;
use utils::ArgGen;

pub struct Catamorphism<Alg: Algebra, Trv: TraversePolicy> {
    dt: Datatype,
    alg: Alg,
    trv: Trv,
}

impl<Alg: Algebra, Trv: TraversePolicy> Catamorphism<Alg, Trv> {

    pub fn new(alg: Alg, trv: Trv, dt: Datatype) -> Self {
        Catamorphism { dt, alg, trv, }
    }

    pub fn codegen(&self) -> TokenStream {
        let dt = &self.dt;

        let dt_ty = self.datatype_type();
        let dt_arg = self.datatype_arg_name();
        let dt_tr_ty = self.transformer_type();
//
        let alg_trait = self.alg_trait_name();
        let alg_struct = self.alg_struct_name();
        let alg_result_ty = self.alg_result_type();
//
        let alg_generics = self.alg_generics();
        let all_generics = self.all_generics();
//
        let alg_generics_bounds = self.alg_generics_bounds();
        let all_generics_bounds = self.all_generics_bounds();
//
        let alg_fields = self.alg_fields();
        let alg_setters = self.alg_setters();
        let alg_inits = self.alg_initializers();
        let alg_match_arms = self.alg_match_arms();

        quote! {
            #dt
//
            struct #alg_struct<'a, #alg_generics> #alg_fields
//
            impl<'a, #alg_generics> #alg_struct<'a, #alg_generics> {
                #(#alg_setters)*
            }
//
            impl<'a, #all_generics> #dt_tr_ty
            for #alg_struct<'a, #alg_generics>
            where #all_generics_bounds {
                fn transform(&self, #dt_arg: #dt_ty) -> #alg_result_ty {
                    match *#dt_arg {
                        #(#alg_match_arms),*
                    }
                }
            }
//
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

    fn datatype_ident(&self) -> Ident {
        self.dt.ident()
    }

    fn datatype_arg_name(&self) -> Ident {
        Ident::new("x", Span::call_site())
    }

    fn datatype_type(&self) -> Type {
        self.trv.datatype_ty(&self.dt)
    }

    fn type_param_type(&self, ty_param: &TypeParam) -> Type {
        self.trv.type_param_type(ty_param)
    }

    fn transformer_type(&self) -> Type {
        let dt_ty = self.datatype_type();
        let r_ty = self.alg_result_type();

        parse_quote! { Transformer<#dt_ty, #r_ty> }
    }

    fn type_param_transformer_type(&self, ty_param: &TypeParam) -> Type {
        let i_ty = self.type_param_type(ty_param);
        let r_ty = self.alg_result_type();

        parse_quote! { Transformer<#i_ty, #r_ty> }
    }

    fn fn_arg_type(&self, field: &Field) -> Type {
        self.trv.fn_arg_type(field)
    }

    fn alg_trait_name(&self) -> Ident {
        self.alg.trait_ident(&self.dt)
    }

    fn alg_struct_name(&self) -> Ident {
        self.alg.struct_ident(&self.dt)
    }

    fn alg_result_type(&self) -> Type {
        self.alg.result_type(&self.dt)
    }

    fn alg_generics(&self) -> Punctuated<GenericParam, Comma> {
        self.alg.generics(&self.dt)
    }

    fn all_generics(&self) -> Punctuated<GenericParam, Comma> {
        let lifetimes = self.trv.lifetimes(&self.dt);
        let alg_generics = self.alg_generics();

        parse_quote! { #(#lifetimes ,)* #(#alg_generics),* }
    }

    fn alg_generics_bounds(&self) -> Punctuated<WherePredicate, Comma> {
        self.alg.generics_bounds(&self.dt)
    }

    fn all_generics_bounds(&self) -> Punctuated<WherePredicate, Comma> {
        let lifetimes_bounds = self.trv.lifetimes_bounds(&self.dt);
        let alg_generics_bounds = self.alg.generics_bounds(&self.dt);

        parse_quote! { #(#lifetimes_bounds ,)* #(#alg_generics_bounds),* }
    }

    fn alg_fields(&self) -> FieldsNamed {
        let mut tranformer_fields = self.iter_type_params(Self::alg_transformer_field);
        let mut variant_fields = self.iter_variants(Self::alg_variant_field);

        let mut fields = Vec::new();
        fields.append(&mut tranformer_fields);
        fields.append(&mut variant_fields);

        let fields = fields.iter()
            .map(|(ident, ty)| {
                quote! { #ident: #ty }
            });

        parse_quote! { { #(#fields),* } }
    }

    fn alg_variant_field(&self, ident: &Ident, fields: &Fields) -> (Ident, Type) {
        let field_fn_ty = self.alg_variant_field_fn_ty(ident, fields);

        (self.alg_variant_field_name(ident), parse_quote! { Box<'a + #field_fn_ty> })
    }

    fn alg_variant_field_name(&self, ident: &Ident) -> Ident {
        self.alg.field_ident(ident)
    }

    fn alg_variant_field_fn_ty(&self, ident: &Ident, fields: &Fields) -> Type {
        let dt_ty = self.datatype_type();
        let r_ty = self.alg_result_type();
        let dt_tr_ty = self.transformer_type();
        let args = fields.iter().map(|field| self.fn_arg_type(field));

        self.trv.fn_type( &parse_quote! { Fn(&#dt_tr_ty, #(#args),*) -> #r_ty } )
    }

    fn alg_transformer_field(&self, ty_param: &TypeParam) -> (Ident, Type) {
        let ident = self.alg_transformer_field_name(&ty_param.ident);

        let transformer_ty = self.type_param_transformer_type(ty_param);
        let typ = parse_quote! { Box<'a + &#transformer_ty> };

        (ident, typ)
    }

    fn alg_transformer_field_name(&self, ident: &Ident) -> Ident {
        Ident::new(
            &format!("tr_{}", ident.to_string().to_lowercase()),
            Span::call_site()
        )
    }

    fn alg_setters(&self) -> Vec<ItemFn> {
        self.iter_variants(Self::alg_setter)
    }

    fn alg_setter(&self, ident: &Ident, fields: &Fields) -> ItemFn {
        let struct_name = self.alg_struct_name();
        let field_name = self.alg_variant_field_name(ident);
        let setter_name = self.alg_setter_name(ident);

        let field_fn_ty = self.alg_variant_field_fn_ty(ident, fields);

        parse_quote! {
            fn #setter_name<'c: 'a, F>(self, f: F) -> Self
            where
                F: 'c + #field_fn_ty
            {
                #struct_name { #field_name: Box::new(f), ..self }
            }
        }
    }

    fn alg_setter_name(&self, ident: &Ident) -> Ident {
        self.alg.setter_ident(ident)
    }

    fn alg_initializers(&self) -> Vec<FieldValue> {
        let mut transformer_initializers = Vec::new(); // self.iter_type_params(Self::alg_transformer_field_initializer);
        let mut variant_initializers = self.iter_variants(Self::alg_variant_field_initializer);

        let mut initializers = Vec::new();
        initializers.append(&mut transformer_initializers);
        initializers.append(&mut variant_initializers);

        initializers
    }

//    fn alg_transformer_field_initializer(&self, ty_param: &TypeParam) -> FieldValue {
//        self.alg.transformer_field_initializer(ty_param)
//    }

    fn alg_variant_field_initializer(&self, ident: &Ident, fields: &Fields) -> FieldValue {
        let field = self.alg_variant_field_name(ident);
        let transformers = self.alg_initializer_transformers();
        let args = self.alg_initializer_args(ident, fields);
        let body = self.alg_initializer_body(ident, &args);

        parse_quote! {
            #field: Box::new(|tr, #(#transformers,)* #(#args,)*| { #body })
        }
    }

    fn alg_initializer_transformers(&self) -> Vec<FnArg> {
        self.iter_type_params(Self::alg_initializer_transformer)
    }

    fn alg_initializer_transformer(&self, ty_param: &TypeParam) -> FnArg {
        let ident = self.alg_transformer_field_name(&ty_param.ident);
        let ty = self.type_param_transformer_type(ty_param);

        parse_quote! { #ident: #ty }
    }

    fn alg_initializer_args(&self, ident: &Ident, fields: &Fields) -> Vec<FnArg> {
        ArgGen::new().iter_fields(fields, |gen, field| {
            gen.fn_arg(&self.trv, field)
        })
    }

    fn alg_initializer_body(&self, ident: &Ident, args: &Vec<FnArg>) -> Expr {
        self.alg.initializer_body(ident, args)
    }

    fn alg_match_arms(&self) -> Vec<Arm> {
        self.iter_variants(Self::alg_match_arm)
    }

    fn alg_match_arm(&self, ident: &Ident, fields: &Fields) -> Arm {
        let field = self.alg_variant_field_name(ident);
        let pat = self.alg_match_pat(ident, fields);
        let args = ArgGen::new().iter_fields(fields, ArgGen::ident);

        parse_quote! {
            #pat => (self.#field)(self, #(#args),*)
        }
    }

    fn alg_match_pat(&self, ident: &Ident, fields: &Fields) -> Pat {
        let dt = self.datatype_ident();
        let args = ArgGen::new().iter_fields(fields, |gen, field| {
            gen.pat(&self.trv, field)
        });

        match fields {
            Fields::Unnamed(_) => {
                parse_quote! {
                    #dt::#ident(#(#args),*)
                }
            },
            _ => unimplemented!(),
        }
    }

    fn iter_variants<'a, R: 'a, F: 'a>(&self, f: F) -> Vec<R>
        where
            F: Fn(&Self, &Ident, &Fields) -> R,
    {
        match self.dt {
            Datatype::Enum(ref item) => item.variants.iter().map(move |variant| {
                f(self, &variant.ident, &variant.fields)
            }),
            _ => unimplemented!()
        }.collect()
    }

    fn iter_type_params<'a, R: 'a, F: 'a>(&self, f: F) -> Vec<R>
        where
            F: Fn(&Self, &TypeParam) -> R,
    {
        match self.dt {
            Datatype::Enum(ref item) => item.generics.type_params().map(move |ty_param| {
                f(self, &ty_param)
            }),
            _ => unimplemented!()
        }.collect()
    }

}