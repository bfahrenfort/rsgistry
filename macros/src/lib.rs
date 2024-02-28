extern crate proc_macro;
use proc_macro::TokenStream;
use proc_quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Field, ItemStruct,
};

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

    TokenStream::from(output)
}

#[proc_macro_derive(Listable)]
pub fn list_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let fields_list = input
        .fields
        .iter()
        .map(|e| e.ident.as_ref().unwrap().to_string()); // If you use unnamed fields in Entry that's on you

    let name = &input.ident;
    let output = quote! {
        impl #name {
            pub fn list_fields<'a>() -> Vec<&'a str> {
                vec![#(#fields_list), *]
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(Keyed)]
pub fn get_key(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let key = input
        .fields
        .iter()
        .filter(|e| e.ident.as_ref().unwrap().to_string().starts_with("UNIQUE"))
        .collect::<Vec<&Field>>()[0]; // There should only be one
                                      // Todo allow multi-identifier Entries
    let key_name = &key.ident.as_ref().unwrap().to_string();

    let name = &input.ident;
    let output = quote! {
        impl #name {
            pub fn get_key<'a>() -> &'a str {
                #key_name
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(FromQueue)]
pub fn from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let mut stmts = Vec::<proc_quote::__rt::TokenStream>::new();
    for field in input.fields.iter() {
        if field.ident.as_ref().unwrap() == "request_type" {
            // Ignore queue-specific field
            continue;
        }
        let name = field.ident.as_ref().unwrap();
        stmts.push(quote! { #name });
    }
    // Todo allow multi-identifier Entries

    let name = &input.ident;
    let output = quote! {
        impl #name {
            pub fn from(q: Queue) -> #name {
                #name {
                    #(#stmts: q.#stmts), * // Cursed
                }
            }
        }
    };

    TokenStream::from(output)
}

struct BindTo(syn::Ident);
impl Parse for BindTo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bind_to = input.parse()?;
        Ok(BindTo(bind_to))
    }
}

#[proc_macro_derive(Bindable, attributes(bind_to))]
pub fn bind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let mut stmts = Vec::<proc_quote::__rt::TokenStream>::new();
    for field in input.fields.iter() {
        let name = field.ident.as_ref().unwrap();
        stmts.push(quote! { #name });
    }

    let name = &input.ident;
    let BindTo(ret_ty) = input
        .attrs
        .iter()
        .find(|a| a.path().segments.len() == 1 && a.path().segments[0].ident == "bind_to")
        .expect("Struct type must have attribute bind_to(QueryReturnType) to derive Bindable!")
        .parse_args()
        .expect("Invalid argument of bind_to");

    let output = quote! {
        impl #name {
            pub fn bind<'q>(
                self: &'q #name,
                query: QueryAs<'q, Postgres, #ret_ty, <Postgres as HasArguments>::Arguments>,
            ) -> QueryAs<'q, Postgres, #ret_ty, <Postgres as HasArguments>::Arguments> {
                query
                    #(.bind(&self.#stmts))*
            }
        }
    };

    TokenStream::from(output)
}
