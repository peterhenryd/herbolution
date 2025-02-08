use crate::util::common_field_derivatives;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Fields, FieldsNamed, GenericParam, Generics, ItemStruct, TypeParam};

pub(super) fn derive(
    ItemStruct { ident, generics, fields, .. }: ItemStruct
) -> TokenStream {
    let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
        panic!("#[derive(Vector)] requires that your structure has named fields.")
    };
    let Generics { lt_token: Some(_), gt_token: Some(_), where_clause: None, .. } = generics else {
        panic!("#[derive(Vector)] requires that your structure has one unbounded type parameter.")
    };
    #[allow(non_snake_case)]
    let Some(GenericParam::Type(TypeParam { ident: T, colon_token: None, eq_token: None, default: None, .. })) = generics.params.first() else {
        panic!("#[derive(Vector)] requires that your structure has one unbounded type parameter.")
    };

    // `fn`: Field name
    // `ft`: Field type

    let (fn_vec, fn_cs_ft, _, fn_col_self_dot_fn) = common_field_derivatives(&fields);
    let mut token_streams = vec![];

    token_streams.push(quote! {
        impl<#T> #ident<#T> {
            pub const fn new(#(#fn_cs_ft)*) -> Self {
                Self { #(#fn_vec),* }
            }
        }
    });

    let component_expr = |index: usize, selected: &TokenStream, default: &TokenStream| {
        fn_vec.iter()
            .enumerate()
            .map(|(i, ts)| if i == index {
                quote! { #ts: #selected }
            } else {
                quote! { #ts: #default }
            })
            .reduce(|acc, e| quote! { #acc, #e })
            .unwrap_or_default()
    };

    let mut remaining_component_fn_len = fields.len();
    while remaining_component_fn_len > 0 {
        remaining_component_fn_len -= 1;

        let inner = component_expr(remaining_component_fn_len, &quote! { #T::one() }, &quote! { #T::zero() });
        let field_name = &fn_vec[remaining_component_fn_len];
        token_streams.push(quote! {
            impl<#T: num::Num> #ident<#T> {
                pub fn #field_name() -> Self { Self { #inner } }
            }
        })
    }

    let neg_inst_inner = fn_vec.iter()
        .map(|ts| quote! { #ts: -self.#ts })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    token_streams.push(quote! {
        impl<#T: std::ops::Neg<Output = #T >> std::ops::Neg for #ident<#T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self { #neg_inst_inner }
            }
        }
    });

    for field_name in &fn_vec {
        let get_field_name = Ident::new(&format!("get_{}", field_name.to_string()), Span::call_site());
        let get_field_name_mut = Ident::new(&format!("{}_mut", get_field_name.to_string()), Span::call_site());
        token_streams.push(quote! {
            impl<#T> #ident<#T> {
                pub fn #get_field_name(&self) -> &#T {
                    &self.#field_name
                }

                pub fn #get_field_name_mut(&mut self) -> &mut #T {
                    &mut self.#field_name
                }
            }
        })
    }

    let clone_inst_inner = fn_col_self_dot_fn.iter()
        .map(|x| quote! { #x.clone() })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let partial_eq_inner = fn_vec.iter()
        .map(|x| quote! { self.#x == other.#x })
        .reduce(|acc, e| quote! { #acc && #e })
        .unwrap_or_default();
    let default_inst_inner = fn_vec.iter()
        .map(|x| quote! { #x: #T::default() })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let hash_inner = fn_vec.iter()
        .map(|x| quote! { self.#x.hash(state); })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    let debug_inner = fn_vec.iter()
        .map(|x| quote! { .field(stringify!(#x), &self.#x) })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    let add_self_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x + rhs.#x })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let add_assign_self_inner = fn_vec.iter()
        .map(|x| quote! { self.#x += rhs.#x; })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    let add_comp_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x + rhs })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let add_assign_comp_inner = fn_vec.iter()
        .map(|x| quote! { self.#x += rhs; })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    let mul_self_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x * rhs.#x })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let mul_comp_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x * rhs })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let sub_self_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x - rhs.#x })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let sub_assign_self_inner = fn_vec.iter()
        .map(|x| quote! { self.#x -= rhs.#x; })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    let sub_comp_inner = fn_vec.iter()
        .map(|x| quote! { #x: self.#x - rhs })
        .reduce(|acc, e| quote! { #acc, #e })
        .unwrap_or_default();
    let sub_assign_comp_inner = fn_vec.iter()
        .map(|x| quote! { self.#x -= rhs; })
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default();
    token_streams.push(quote! {
        impl<#T: Copy> Copy for #ident<#T> {}

        impl<#T: Clone> Clone for #ident<#T> {
            fn clone(&self) -> Self {
                Self { #clone_inst_inner }
            }
        }

        impl<#T: Eq> Eq for #ident<#T> {}

        impl<#T: PartialEq> PartialEq for #ident<#T> {
            fn eq(&self, other: &Self) -> bool { #partial_eq_inner }
        }

        impl<#T: Default> Default for #ident<#T> {
            fn default() -> Self {
                Self { #default_inst_inner }
            }
        }

        impl<#T: std::hash::Hash> std::hash::Hash for #ident<#T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #hash_inner
            }
        }

        impl<#T: std::fmt::Debug> std::fmt::Debug for #ident<#T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #debug_inner
                    .finish()
            }
        }

        impl<#T: Copy + std::ops::Add<Output = #T>> std::ops::Add for #ident<#T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self { #add_self_inner }
            }
        }

        impl<#T: Copy + std::ops::Add<Output = #T>> std::ops::Add<#T> for #ident<#T> {
            type Output = Self;

            fn add(self, rhs: #T) -> Self::Output {
                Self { #add_comp_inner }
            }
        }

        impl<#T: Copy + std::ops::Mul<Output = #T>> std::ops::Mul for #ident<#T> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self { #mul_self_inner }
            }
        }

        impl<#T: Copy + std::ops::Mul<Output = #T>> std::ops::Mul<#T> for #ident<#T> {
            type Output = Self;

            fn mul(self, rhs: #T) -> Self::Output {
                Self { #mul_comp_inner }
            }
        }

        impl<#T: Copy + std::ops::AddAssign> std::ops::AddAssign for #ident<#T> {
            fn add_assign(&mut self, rhs: Self) {
                #add_assign_self_inner
            }
        }

        impl<#T: Copy + std::ops::AddAssign> std::ops::AddAssign<#T> for #ident<#T> {
            fn add_assign(&mut self, rhs: #T) {
                #add_assign_comp_inner
            }
        }

        impl<#T: Copy + std::ops::Sub<Output = #T>> std::ops::Sub for #ident<#T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self { #sub_self_inner }
            }
        }

        impl<#T: Copy + std::ops::Sub<Output = #T>> std::ops::Sub<#T> for #ident<#T> {
            type Output = Self;

            fn sub(self, rhs: #T) -> Self::Output {
                Self { #sub_comp_inner }
            }
        }

        impl<#T: Copy + std::ops::SubAssign> std::ops::SubAssign for #ident<#T> {
            fn sub_assign(&mut self, rhs: Self) {
                #sub_assign_self_inner
            }
        }

        impl<#T: Copy + std::ops::SubAssign> std::ops::SubAssign<#T> for #ident<#T> {
            fn sub_assign(&mut self, rhs: #T) {
                #sub_assign_comp_inner
            }
        }
    });

    token_streams.into_iter()
        .reduce(|acc, e| quote! { #acc #e })
        .unwrap_or_default()
}