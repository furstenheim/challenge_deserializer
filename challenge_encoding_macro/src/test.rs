#[cfg(test)]
mod tests {
    use crate::ChallengeEncoding;
    use challenge_encoding_derive::ChallengeEncoding;

    #[derive(ChallengeEncoding)]
    struct Struct;

    #[test]
    fn it_works() {
        let s = Struct {};
        s.deserialize("a");
        assert_eq!(2 + 2, 4);
    }
}