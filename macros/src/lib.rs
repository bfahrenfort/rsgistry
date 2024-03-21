extern crate proc_macro;
use proc_macro::TokenStream;
use proc_quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    ItemStruct, Token,
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

struct Keys(Punctuated<syn::Ident, Token![,]>);
impl Parse for Keys {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keys = input.parse_terminated(syn::Ident::parse, Token![,])?;
        Ok(Keys(keys))
    }
}

#[proc_macro_derive(Keyed, attributes(keys))]
pub fn get_keys(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let Keys(keys) = input
        .attrs
        .iter()
        .find(|a| a.path().segments.len() == 1 && a.path().segments[0].ident == "keys")
        .expect("Struct type must have attribute keys(param1, ...) to derive Keyed!")
        .parse_args()
        .expect("Invalid argument of macro attribute keys");

    let keys: Vec<String> = keys.into_iter().map(|e| e.to_string()).collect();
    let key_fields: Vec<syn::Field> = input
        .fields
        .into_iter()
        .filter(|f| keys.contains(&f.ident.as_ref().unwrap().to_string()))
        .collect();

    let name = &input.ident;
    let key_types = key_fields
        .iter()
        .map(|f| f.ty.clone())
        .collect::<Vec<syn::Type>>();
    let key_idents = key_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<syn::Ident>>();
    let key_names = key_idents
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    let tuple_name = syn::Ident::new(&format!("{}KeyTuple", name), name.span());
    let tuple_struct_name = syn::Ident::new(&format!("{}KeyTupleStruct", name), name.span());
    let len = keys.len();
    let output = quote! {
        pub type #tuple_name = (#(#key_types), *);
        pub struct #tuple_struct_name(#(pub #key_types), *);
        impl #name {
            pub fn get_keys() -> [&'static str; #len] {
                [#(#keys), *]
            }

            // Cursed
            pub fn yeet_tuple((#(#key_idents), *): #tuple_name) -> #tuple_struct_name {
                #tuple_struct_name(
                    #(#key_idents), *
                )
            }

            pub fn transform_fetch_route(o: aide::transform::TransformOperation) -> aide::transform::TransformOperation {
                o
                    #(.parameter::<#key_types, _>(#key_names, |op| {op.description("A unique constraint on the entry")}))*
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
        .expect("Invalid argument of macro attribute bind_to");

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
