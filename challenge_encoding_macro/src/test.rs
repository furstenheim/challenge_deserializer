#[cfg(test)]
mod tests {
    use crate::ChallengeEncoding;
    use challenge_encoding_derive::ChallengeEncoding;

    #[derive(ChallengeEncoding)]
    struct Struct {
        active: bool,
    }

    #[test]
    fn it_works() {
        let s = Struct {active: true};
        s.deserialize("a");
        assert_eq!(2 + 2, 4);
    }
}