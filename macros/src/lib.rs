extern crate proc_macro;
use proc_macro::TokenStream;
use proc_quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Countable)]
pub fn field_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let field_count = input.fields.iter().count();

    let name = &input.ident;
    let output = quote! {
        impl #name {
            pub fn field_count() -> usize {
                #field_count
            }
        }
    };

    // Return output tokenstream
    TokenStream::from(output)
}

#[proc_macro_derive(Listable)]
pub fn list_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let fields_list = input
        .fields
        .iter()
        .map(|e| e.ident.as_ref().unwrap().to_string().to_owned()) // If you use unnamed fields in Entry that's on you
        .collect::<Vec<String>>()
        .join(", ");

    let name = &input.ident;
    let output = quote! {
        impl #name {
            pub fn list_fields<'q>() -> &'q str {
                #fields_list
            }
        }
    };

    // Return output tokenstream
    TokenStream::from(output)
}
