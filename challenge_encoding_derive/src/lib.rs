extern crate proc_macro;
mod test;

use syn::Fields::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(ChallengeEncoding)]
pub fn challenge_encoding_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse2(TokenStream::from(input)).unwrap();
    return proc_macro::TokenStream::from(impl_challenge_encoding(&ast));
}

fn impl_challenge_encoding(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut message: String = "------".to_owned();

    match &ast.data {
        syn::Data::Struct(ds) => {
            data_struct(ds);
            println!("1111. {}", syn::Error::new_spanned(ast, "struct are not supported"))
        },
        syn::Data::Enum(_) => println!("22222. {}", syn::Error::new_spanned(ast, "enums are not supported")),

        syn::Data::Union(_) => println!("3333. {}", syn::Error::new_spanned(ast, "unions are not supported")),
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

fn data_struct (ds: &syn::DataStruct) -> () {
    println!("length of fields: {}", ds.fields.len());
    for field in ds.fields.iter() {
        match field.ty {
            syn::Type::Array(_) => panic!("Array error"),
            syn::Type::BareFn(_) => panic!("BareFn error"),
            syn::Type::Group(_) => panic!("Group error"),
            syn::Type::ImplTrait(_) => panic!("ImplTrait error"),
            syn::Type::Infer(_) => panic!("Infer error"),
            syn::Type::Macro(_) => panic!("Macro error"),
            syn::Type::Never(_) => panic!("Never error"),
            syn::Type::Paren(_) => panic!("Paren error"),
            syn::Type::Path(ref a) => {
                println!("{:?}", 1);
            },
            syn::Type::Ptr(_) => panic!("Ptr error"),
            syn::Type::Reference(_) => panic!("Reference error"),
            syn::Type::Slice(_) => panic!("Slice error"),
            syn::Type::TraitObject(_) => panic!("TraitObject error"),
            syn::Type::Tuple(_) => panic!("Tuple error"),
            syn::Type::Verbatim(_) => panic!("Verbatim error"),
            _ => panic!("aaaaaa"),
        };
    }

}