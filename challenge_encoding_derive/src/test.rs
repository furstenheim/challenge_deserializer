#[cfg(test)]
mod tests {
    use proc_macro2;
    use syn::{DeriveInput, parse};
    use syn::Expr;
    use proc_macro2::TokenStream;
    use syn::Token;
    use crate::impl_challenge_encoding;
    use super::*;

    #[test]
    fn it_works() {
        let str = "
struct Struct {
        active: bool,
    }
        ";
        let stream: TokenStream = str.parse().unwrap();
        let ast = syn::parse2::<DeriveInput>(stream).unwrap();
        impl_challenge_encoding(&ast);
        assert_eq!(2 + 2, 4);
    }
}