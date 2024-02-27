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
