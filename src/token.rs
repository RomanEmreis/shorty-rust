use std::fmt;

pub struct Token {
    value: String,
}

impl Token {
    const MIN_VALUE: u64 = 56_800_235_584;
    const MAX_VALUE: u64 = 3_521_614_606_207;

    const DEFAULT_LENGTH: usize = 7;
    const BASE: u64 = 62;

    const CHARS: &'static str = "QoNPMlEDkABC06789zxyvwustrq21453pOnmLKjZYXWVUTSRihgfedcbJIHGFa";

    pub fn new(count: u64) -> Self {
        if count < Self::MIN_VALUE || count > Self::MAX_VALUE {
            panic!("count out of range");
        }

        let mut token = vec![' '; Self::DEFAULT_LENGTH];
        let mut count = count;
        let chars: Vec<char> = Self::CHARS.chars().collect();

        let mut j = Self::DEFAULT_LENGTH;
        while count != 0 {
            j -= 1;
            let i = (count % Self::BASE) as usize;
            token[j] = chars[i];
            count /= Self::BASE;
        }

        let value = token.iter().collect();
        Token { value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        token.value
    }
}