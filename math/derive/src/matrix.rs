use crate::util::common_field_derivatives;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, FieldsNamed, GenericParam, Generics, ItemStruct, TypeParam};

pub(super) fn derive(
    ItemStruct { ident, generics, fields, .. }: ItemStruct
) -> TokenStream {
    let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
        panic!("#[derive(Matrix)] requires that your structure has named fields.")
    };
    let Generics { lt_token: Some(_), gt_token: Some(_), where_clause: None, .. } = generics else {
        panic!("#[derive(Matrix)] requires that your structure has one unbounded type parameter.")
    };
    let Some(GenericParam::Type(TypeParam { ident: generic, colon_token: None, eq_token: None, default: None, .. })) = generics.params.first() else {
        panic!("#[derive(Matrix)] requires that your structure has one unbounded type parameter.")
    };

    let (fn_vec, fn_cs_ft, _, fn_col_self_dot_fn) = common_field_derivatives(&fields);

    let mut token_streams = vec![];

    let fn_cs_ft_ts = fn_cs_ft.iter().cloned().reduce(|acc, e| quote! { #acc #e }).unwrap_or_default();
    let fn_ts = fn_vec.iter().cloned().reduce(|acc, e| quote! { #acc, #e }).unwrap_or_default();
    token_streams.push(quote! {
        impl<#generic> #ident<#generic> {
            pub const fn new(#fn_cs_ft_ts) -> Self {
                Self { #fn_ts }
            }
        }
    });

    let clone_inst_inner = fn_col_self_dot_fn.iter()
        .map(|ts| quote! { #ts.clone() })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let partial_eq_inner = fn_vec.iter()
        .map(|ts| quote! { self.#ts == other.#ts })
        .reduce(|acc, e| quote! { #acc && #e })
        .unwrap_or_default();
    token_streams.push(quote! {
        impl<#generic: Copy> Copy for #ident<#generic> {}

        impl<#generic: Clone> Clone for #ident<#generic> {
            fn clone(&self) -> Self {
                Self { #clone_inst_inner }
            }
        }

        impl<#generic: Eq> Eq for #ident<#generic> {}

        impl<#generic: PartialEq> PartialEq for #ident<#generic> {
            fn eq(&self, other: &Self) -> bool {
                #partial_eq_inner
            }
        }
    });

    token_streams.into_iter()
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default()
}