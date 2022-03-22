extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(ChallengeEncoding)]
pub fn challenge_encoding_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_challenge_encoding(&ast);
}

fn impl_challenge_encoding(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut message: String = "------".to_owned();

    match &ast.data {
        Data::Struct(_) => {
            println!("1111. {}", syn::Error::new_spanned(ast, "struct are not supported"))
        },
        Data::Enum(_) => println!("22222. {}", syn::Error::new_spanned(ast, "enums are not supported")),

        Data::Union(_) => println!("3333. {}", syn::Error::new_spanned(ast, "unions are not supported")),
    };
    for _attr in ( & ast).attrs.iter() {
        message.push_str(stringify ! (attr));
    }
    message.push_str("a");
    let gen = quote! {
        impl ChallengeEncoding for #name {
            fn deserialize<'a>(self, a: &str) {

                println!("{}", #message);
            }
        }
    };
return gen.into();
}