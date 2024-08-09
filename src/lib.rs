extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataEnum};


/// A derive macro that implements `Hash`, `PartialEq`, and `Eq` based on enum discriminants.
/// 
/// This macro ensures that hashing and equality checks are done based only on the enum variant type,
/// ignoring any associated data. This is particularly useful for using enums with custom data in
/// collections like `HashSet` or as keys in `HashMap`, where equality and hashing are determined by
/// the variant type alone.
///
#[proc_macro_derive(DiscriminantHashEq)]
pub fn discriminant_hash_eq_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type we are deriving
    let name = input.ident;

    // Ensure it's an enum
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("#[derive(DiscriminantHashEq)] is only defined for enums."),
    };

    // Generate the implementation
    let gen = impl_discriminant_hash_eq(&name, &data);

    // Return the generated implementation
    gen.into()
}

fn impl_discriminant_hash_eq(name: &syn::Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    let variants = &data.variants;

    // Generate the match arms for the discriminant
    let match_arms = variants.iter().map(|variant| {
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
    }
}
