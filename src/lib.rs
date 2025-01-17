extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

/// A derive macro that implements `Ord`, `PartialOrd`, `Eq`, and `PartialEq`
/// based on the discriminants of an enum, ignoring any associated data,
/// and also provides a `variant_index` method via a `VariantIndex` trait.
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
    // Generate a match arm for each variant with its index
    let variant_indices = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name { .. } => #index,
            }
        });

    // Generate the implementation of `Ord`, `PartialOrd`, `Eq`, and `PartialEq` and the `VariantIndex` trait
    quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                let self_index = self.variant_index();
                let other_index = other.variant_index();

                self_index == other_index
            }
        }

        impl Eq for #name {}

        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                let self_index = self.variant_index();
                let other_index = other.variant_index();

                Some(self_index.cmp(&other_index))
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let self_index = self.variant_index();
                let other_index = other.variant_index();

                self_index.cmp(&other_index)
            }
        }

        /// A trait to provide variant indexing for enums.
        pub trait VariantIndex {
            fn variant_index(&self) -> usize;
        }

        impl VariantIndex for #name {
            fn variant_index(&self) -> usize {
                match self {
                    #(#variant_indices)*
                }
            }
        }
    }
}
