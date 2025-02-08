use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Field;

pub fn common_field_derivatives(fields: &Punctuated<Field, Comma>) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let fn_vec = fields.iter()
        .map(|Field { ident, .. }| ident.as_ref().unwrap())
        .map(|i| quote! { #i })
        .collect::<Vec<_>>();
    let fn_cs_ft = fields.iter()
        .map(|Field { ident, ty, .. }| (ident.as_ref().unwrap(), ty))
        .map(|(ident, ty)| quote! {
            #ident: #ty,
        })
        .collect::<Vec<_>>();
    let self_dot_fn = fn_vec.iter()
        .map(|x| quote! { self.#x })
        .collect::<Vec<_>>();
    let fn_col_self_dot_fn = fn_vec.iter()
        .zip(self_dot_fn.iter())
        .map(|(i, self_dot_fn)| quote! { #i: #self_dot_fn })
        .collect::<Vec<_>>();

    (fn_vec, fn_cs_ft, self_dot_fn, fn_col_self_dot_fn)
}