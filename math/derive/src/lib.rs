mod matrix;
mod scalar;
mod util;
mod vector;
mod implementor;

#[proc_macro_derive(Matrix)]
pub fn derive_matrix(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    matrix::derive(syn::parse_macro_input!(ts as syn::ItemStruct)).into()
}

#[proc_macro_derive(Scalar)]
pub fn derive_scalar(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    scalar::derive(syn::parse_macro_input!(ts as syn::ItemStruct)).into()
}

#[proc_macro_derive(Vector)]
pub fn derive_vector(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    vector::derive(syn::parse_macro_input!(ts as syn::ItemStruct)).into()
}