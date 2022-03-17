extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ChallengeEncoding)]
pub fn challenge_encoding_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_challenge_encoding(&ast);
}

fn impl_challenge_encoding(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ChallengeEncoding for #name {
            fn deserialize<'a>(self, a: &str) {
                println!("Hello there. I received {}", a);
            }
        }
    };
    return gen.into();
}