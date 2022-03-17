mod test;

pub trait ChallengeEncoding {
    fn deserialize<'input>(self, input: &str);
}
