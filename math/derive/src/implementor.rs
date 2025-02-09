use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Fields, GenericParam, ItemStruct, LitInt, TypeParam};

pub struct Implementor {
    pub type_name_ident: Ident,
    pub generic_param_ident: Ident,
    pub field_idents: Vec<TokenStream>,
    pub is_tuple: bool,
    pub token_streams: Vec<TokenStream>,
}

impl Implementor {
    pub fn new(macro_name: &str, ItemStruct { ident, generics, fields, .. }: ItemStruct) -> Self {
        if fields.len() == 0 {
            panic!("#[derive({macro_name})] requires that \"{ident}\" must have at least one field.");
        }

        let Some(
            GenericParam::Type(
                TypeParam {
                    ident: generic_param_ident,
                    colon_token: None,
                    eq_token: None,
                    default: None,
                    ..
                }
            )
        ) = generics.params.into_iter().next() else {
            panic!("#[derive({macro_name})] requires that \"{ident}\" must have one unbounded type parameter.");
        };

        Self {
            type_name_ident: ident,
            generic_param_ident,
            field_idents: fields.iter()
                .enumerate()
                .map(|(i, f)| match &f.ident {
                    None => LitInt::new(&i.to_string(), Span::call_site()).to_token_stream(),
                    Some(ident) => ident.to_token_stream(),
                })
                .collect::<Vec<_>>(),
            is_tuple: matches!(fields, Fields::Unnamed(_)),
            token_streams: vec![],
        }
    }

    pub fn construct(&self, expr: impl Fn(&TokenStream, usize) -> TokenStream) -> TokenStream {
        let exprs = self.get_constructor_tokens(expr);
        if self.is_tuple {
            quote! { Self(#exprs) }
        } else {
            quote! { Self { #exprs } }
        }
    }

    pub fn get_constructors(&self, expr: impl Fn(&TokenStream, usize) -> TokenStream) -> impl Iterator<Item=TokenStream> {
        self.field_idents.iter()
            .enumerate()
            .map(move |(i, x)| {
                let expr = expr(x, i);
                if self.is_tuple {
                    quote! { #expr }
                } else {
                    quote! { #x: #expr }
                }
            })
    }

    pub fn get_constructor_tokens(&self, expr: impl Fn(&TokenStream, usize) -> TokenStream) -> TokenStream {
        self.get_constructors(expr)
            .reduce(|acc, e| quote! { #acc, #e })
            .unwrap_or_default()
    }

    pub fn get_parameter_names(&self) -> Vec<TokenStream> {
        if self.is_tuple {
            self.field_idents.iter()
                .map(|x| Ident::new(
                    &format!("p{}", x.to_string()),
                    Span::call_site(),
                ).to_token_stream())
                .collect()
        } else {
            self.field_idents.iter()
                .map(|x| quote! { #x })
                .collect()
        }
    }

    pub fn impl_constructor(&mut self) -> &mut Self {
        let parameter_names = self.get_parameter_names();
        let gpi = &self.generic_param_ident;
        let parameters = parameter_names.iter()
            .map(move |x| quote! { #x: #gpi })
            .reduce(|acc, e| quote! { #acc, #e })
            .unwrap_or_default();
        let construction = self.construct(|_, i| parameter_names[i].to_token_stream());
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        self.token_streams.push(quote! {
            impl<#gpi> #tni<#gpi> {
                pub const fn new(#parameters) -> Self {
                    #construction
                }
            }
        });

        self
    }

    pub fn impl_copy(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        self.token_streams.push(quote! {
            impl<#gpi: Copy> Copy for #tni<#gpi> {}
        });

        self
    }

    pub fn impl_clone(&mut self) -> &mut Self {
        let construction = self.construct(|ident, _| quote! { self.#ident.clone() });
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        self.token_streams.push(quote! {
            impl<#gpi: Clone> Clone for #tni<#gpi> {
                fn clone(&self) -> Self {
                    #construction
                }
            }
        });

        self
    }

    pub fn impl_eq(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        self.token_streams.push(quote! {
            impl<#gpi: Eq> Eq for #tni<#gpi> {}
        });

        self
    }

    pub fn impl_partial_eq(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let exprs = field_idents.iter()
            .map(|ts| quote! { self.#ts == other.#ts })
            .reduce(|acc, e| quote! { #acc && #e })
            .unwrap_or_default();

        self.token_streams.push(quote! {
            impl<#gpi: PartialEq> PartialEq for #tni<#gpi> {
                fn eq(&self, other: &Self) -> bool {
                    #exprs
                }
            }
        });

        self
    }

    pub fn impl_ord(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let field = field_idents[0].clone();

        self.token_streams.push(quote! {
            impl<#gpi: Ord> Ord for #tni<#gpi> {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.#field.cmp(&other.#field)
                }
            }
        });

        self
    }

    pub fn impl_partial_ord(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let field = field_idents[0].clone();

        self.token_streams.push(quote! {
            impl<#gpi: PartialOrd> PartialOrd for #tni<#gpi> {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.#field.partial_cmp(&other.#field)
                }
            }
        });

        self
    }

    pub fn impl_default(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let exprs = field_idents.iter()
            .map(|ident| quote! { #ident: #gpi::default() })
            .reduce(|acc, e| quote! { #acc, #e })
            .unwrap_or_default();

        self.token_streams.push(quote! {
            impl<#gpi: Default> Default for #tni<#gpi> {
                fn default() -> Self {
                    Self { #exprs }
                }
            }
        });

        self
    }

    pub fn impl_hash(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let stmts = field_idents.iter()
            .map(|ident| quote! { #ident.hash(state); })
            .reduce(|acc, e| quote! { #acc #e })
            .unwrap_or_default();

        self.token_streams.push(quote! {
            impl<#gpi: std::hash::Hash> std::hash::Hash for #tni<#gpi> {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    #stmts
                }
            }
        });

        self
    }

    pub fn impl_debug(&mut self) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;

        let mut token_stream = TokenStream::new();
        if self.is_tuple {
            quote! { f.debug_tuple(stringify!(#tni)) }.to_tokens(&mut token_stream);
            for ident in field_idents.iter() {
                quote! { .field(&self.#ident) }.to_tokens(&mut token_stream);
            }
        } else {
            quote! { f.debug_struct(stringify!(#tni)) }.to_tokens(&mut token_stream);
            for ident in field_idents.iter() {
                quote! { .field(stringify!(#ident), &self.#ident) }.to_tokens(&mut token_stream);
            }
        }

        self.token_streams.push(quote! {
            impl<#gpi: std::fmt::Debug> std::fmt::Debug for #tni<#gpi> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #token_stream.finish()
                }
            }
        });

        self
    }

    pub fn impl_binary_op(&mut self, op: &str, ty: &str, expr: impl Fn(&TokenStream) -> TokenStream) -> &mut Self {
        let construction = self.construct(|ts, _| expr(ts));
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        let op_fn = Ident::new(&op.to_snake(), Span::call_site());
        let op = Ident::new(op, Span::call_site());
        let ty = Ident::new(ty, Span::call_site());

        self.token_streams.push(quote! {
            impl<#gpi: std::ops::#op<Output = #gpi>> std::ops::#op<#ty> for #tni<#gpi> {
                type Output = Self;

                fn #op_fn(self, rhs: #ty) -> Self::Output {
                    #construction
                }
            }
        });

        self
    }

    pub fn impl_binary_assign_op(&mut self, op: &str, ty: &str, expr: impl Fn(&TokenStream) -> TokenStream) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let exprs = field_idents.iter()
            .map(expr)
            .map(|ts| quote! { #ts; })
            .reduce(|acc, e| quote! { #acc #e })
            .unwrap_or_default();

        let op_fn = Ident::new(&op.to_snake(), Span::call_site());
        let op = Ident::new(op, Span::call_site());
        let ty = Ident::new(ty, Span::call_site());

        self.token_streams.push(quote! {
            impl<#gpi: std::ops::#op> std::ops::#op<#ty> for #tni<#gpi> {
                fn #op_fn(&mut self, rhs: #ty) {
                    #exprs
                }
            }
        });

        self
    }

    pub fn impl_unary_op(&mut self, op: &str, expr: impl Fn(&TokenStream) -> TokenStream) -> &mut Self {
        let construction = self.construct(|ts, _| expr(ts));
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;

        let op_fn = Ident::new(&op.to_snake(), Span::call_site());
        let op = Ident::new(op, Span::call_site());

        self.token_streams.push(quote! {
            impl<#gpi: std::ops::#op<Output = #gpi>> std::ops::#op for #tni<#gpi> {
                type Output = Self;

                fn #op_fn(self) -> Self::Output {
                    #construction
                }
            }
        });

        self
    }

    pub fn impl_add_self(&mut self) -> &mut Self {
        self.impl_binary_op("Add", "Self", |x| quote! { self.#x + rhs.#x })
    }

    pub fn impl_add_component(&mut self) -> &mut Self {
        self.impl_binary_op("Add", &self.generic_param_ident.to_string(), |x| quote! { self.#x + rhs })
    }

    pub fn impl_add_assign_self(&mut self) -> &mut Self {
        self.impl_binary_assign_op("AddAssign", "Self", |x| quote! { self.#x += rhs.#x })
    }

    pub fn impl_add_assign_component(&mut self) -> &mut Self {
        self.impl_binary_assign_op("AddAssign", &self.generic_param_ident.to_string(), |x| quote! { self.#x += rhs })
    }

    pub fn impl_sub_self(&mut self) -> &mut Self {
        self.impl_binary_op("Sub", "Self", |x| quote! { self.#x - rhs.#x })
    }

    pub fn impl_sub_component(&mut self) -> &mut Self {
        self.impl_binary_op("Sub", &self.generic_param_ident.to_string(), |x| quote! { self.#x - rhs })
    }

    pub fn impl_sub_assign_self(&mut self) -> &mut Self {
        self.impl_binary_assign_op("SubAssign", "Self", |x| quote! { self.#x -= rhs.#x })
    }

    pub fn impl_sub_assign_component(&mut self) -> &mut Self {
        self.impl_binary_assign_op("SubAssign", &self.generic_param_ident.to_string(), |x| quote! { self.#x -= rhs })
    }

    pub fn impl_mul_self(&mut self) -> &mut Self {
        self.impl_binary_op("Mul", "Self", |x| quote! { self.#x * rhs.#x })
    }

    pub fn impl_mul_component(&mut self) -> &mut Self {
        self.impl_binary_op("Mul", &self.generic_param_ident.to_string(), |x| quote! { self.#x * rhs })
    }

    pub fn impl_mul_assign_self(&mut self) -> &mut Self {
        self.impl_binary_assign_op("MulAssign", "Self", |x| quote! { self.#x *= rhs.#x })
    }

    pub fn impl_mul_assign_component(&mut self) -> &mut Self {
        self.impl_binary_assign_op("MulAssign", &self.generic_param_ident.to_string(), |x| quote! { self.#x *= rhs })
    }

    pub fn impl_div_self(&mut self) -> &mut Self {
        self.impl_binary_op("Div", "Self", |x| quote! { self.#x / rhs.#x })
    }

    pub fn impl_div_component(&mut self) -> &mut Self {
        self.impl_binary_op("Div", &self.generic_param_ident.to_string(), |x| quote! { self.#x / rhs })
    }

    pub fn impl_div_assign_self(&mut self) -> &mut Self {
        self.impl_binary_assign_op("DivAssign", "Self", |x| quote! { self.#x /= rhs.#x })
    }

    pub fn impl_div_assign_component(&mut self) -> &mut Self {
        self.impl_binary_assign_op("DivAssign", &self.generic_param_ident.to_string(), |x| quote! { self.#x /= rhs })
    }

    pub fn impl_rem_self(&mut self) -> &mut Self {
        self.impl_binary_op("Rem", "Self", |x| quote! { self.#x % rhs.#x })
    }

    pub fn impl_rem_component(&mut self) -> &mut Self {
        self.impl_binary_op("Rem", &self.generic_param_ident.to_string(), |x| quote! { self.#x % rhs })
    }

    pub fn impl_rem_assign_self(&mut self) -> &mut Self {
        self.impl_binary_assign_op("RemAssign", "Self", |x| quote! { self.#x %= rhs.#x })
    }

    pub fn impl_rem_assign_component(&mut self) -> &mut Self {
        self.impl_binary_assign_op("RemAssign", &self.generic_param_ident.to_string(), |x| quote! { self.#x %= rhs })
    }

    pub fn impl_neg(&mut self) -> &mut Self {
        self.impl_unary_op("Neg", |x| quote! { -self.#x })
    }

    pub fn impl_not(&mut self) -> &mut Self {
        self.impl_unary_op("Not", |x| quote! { !self.#x })
    }

    /*
    pub fn impl_map_function(&mut self, name: &str, params: TokenStream, bounds: Option<TokenStream>, f: impl Fn(&TokenStream, usize) -> TokenStream) -> &mut Self {
        let name = Ident::new(name, Span::call_site());
        let exprs = self.get_constructor_tokens(f);
        let bounds = bounds.map(|x| quote! { : #x }).unwrap_or_default();
        let Self { type_name_ident: tni, generic_param_ident: gpi, .. } = self;
        self.token_streams.push(quote! {
            impl<#gpi #bounds> #tni<#gpi> {
                pub fn #name(#params) -> Self {
                    Self { #exprs }
                }
            }
        });

        self
    }

    pub fn impl_method_for_field(&mut self, f: impl Fn(&TokenStream) -> TokenStream) -> &mut Self {
        let Self { type_name_ident: tni, generic_param_ident: gpi, field_idents, .. } = self;
        let exprs = field_idents.iter()
            .map(f)
            .map(|ts| quote! { #ts; })
            .reduce(|acc, e| quote! { #acc #e })
            .unwrap_or_default();

        self.token_streams.push(quote! {
            impl<#gpi> #tni<#gpi> {
                pub fn method(&mut self) {
                    #exprs
                }
            }
        });

        self
    }
     */
}

impl ToTokens for Implementor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token_stream in &self.token_streams {
            token_stream.to_tokens(tokens);
        }
    }
}