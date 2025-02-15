use crate::implementor::Implementor;
use quote::ToTokens;

pub fn derive(item_struct: syn::ItemStruct) -> proc_macro2::TokenStream {
    Implementor::new("Scalar", item_struct)
        .impl_constructor()
        .impl_copy()
        .impl_clone()
        .impl_eq()
        .impl_partial_eq()
        .impl_ord()
        .impl_partial_ord()
        .impl_default()
        .impl_hash()
        .impl_debug()
        .impl_add_self()
        .impl_add_assign_self()
        .impl_add_component()
        .impl_add_assign_component()
        .impl_sub_self()
        .impl_sub_assign_self()
        .impl_sub_component()
        .impl_sub_assign_component()
        .impl_mul_self()
        .impl_mul_assign_self()
        .impl_mul_component()
        .impl_mul_assign_component()
        .impl_div_self()
        .impl_div_assign_self()
        .impl_div_component()
        .impl_div_assign_component()
        .impl_rem_self()
        .impl_rem_assign_self()
        .impl_rem_component()
        .impl_rem_assign_component()
        .impl_neg()
        .impl_not()
        .to_token_stream()
}