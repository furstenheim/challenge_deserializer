#[cfg(test)]
mod tests {
    use proc_macro2;
    use syn::{DeriveInput};
    use proc_macro2::TokenStream;
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
        let stream1 = impl_challenge_encoding(&ast);
        println!("{}", stream1);
        assert_eq!(2 + 2, 4);
    }
}