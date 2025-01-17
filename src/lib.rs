extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataEnum};

/// A derive macro that implements `Ord`, `PartialOrd`, `Eq`, `PartialEq`, and `Hash`
/// based on the discriminants of an enum, ignoring any associated data.
///
/// This macro ensures that comparisons, equality checks, and hashing are done solely
/// based on the enum variant type.
#[proc_macro_derive(DiscriminantOrdEq)]
pub fn discriminant_ord_eq_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type we are deriving
    let name = input.ident;

    // Ensure it's an enum
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("#[derive(DiscriminantOrdEq)] is only defined for enums."),
    };

    // Generate the implementation
    let gen = impl_discriminant_ord_eq(&name, &data);

    // Return the generated implementation
    gen.into()
}

fn impl_discriminant_ord_eq(name: &syn::Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    // Generate match arms for accessing the discriminant
    let match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                quote! {
                    #name::#variant_name { .. } => std::mem::discriminant(self),
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #name::#variant_name => std::mem::discriminant(self),
                }
            }
        }
    });

    quote! {
        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let discriminant = match self {
                    #(#match_arms)*
                };
                discriminant.hash(state);
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }

        impl Eq for #name {}

        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(std::mem::discriminant(self).cmp(&std::mem::discriminant(other)))
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                std::mem::discriminant(self).cmp(&std::mem::discriminant(other))
            }
        }
    }
}
