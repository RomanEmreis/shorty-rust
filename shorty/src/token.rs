use std::fmt;
use std::io::{Error, ErrorKind};

pub(crate) const MIN_VALUE: u64 = 56_800_235_584;
const MAX_VALUE: u64 = 3_521_614_606_207;
const DEFAULT_LENGTH: usize = 7;
const BASE: u64 = 62;
const CHARS: &[u8] = b"QoNPMlEDkABC06789zxyvwustrq21453pOnmLKjZYXWVUTSRihgfedcbJIHGFa";

pub(crate) struct Token {
    value: [u8; DEFAULT_LENGTH],
}

impl Token {
    pub fn new(count: u64) -> Result<Self, Error> {
        if !(MIN_VALUE..=MAX_VALUE).contains(&count) {
            return Err(Error::new(ErrorKind::InvalidData, "count out of range"));
        }
        let mut token = [b'\0'; DEFAULT_LENGTH];
        let mut count = count;
        let mut j = DEFAULT_LENGTH;
        while count != 0 {
            j -= 1;
            let i = (count % BASE) as usize;
            token[j] = CHARS[i];
            count /= BASE;
        }
        Ok(Token { value: token })
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = String::from_utf8_lossy(&self.value); 
        write!(f, "{str}")
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        String::from_utf8_lossy(&token.value).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn test_token_new_valid_min_value() {
        let token = Token::new(MIN_VALUE).unwrap();
        // The token should be created successfully
        assert_eq!(token.value.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn test_token_new_valid_max_value() {
        let token = Token::new(MAX_VALUE).unwrap();
        // The token should be created successfully
        assert_eq!(token.value.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn test_token_new_valid_mid_value() {
        let mid_value = (MIN_VALUE + MAX_VALUE) / 2;
        let token = Token::new(mid_value).unwrap();
        assert_eq!(token.value.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn test_token_new_below_min_value() {
        let result = Token::new(MIN_VALUE - 1);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
            assert_eq!(e.to_string(), "count out of range");
        }
    }

    #[test]
    fn test_token_new_above_max_value() {
        let result = Token::new(MAX_VALUE + 1);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
            assert_eq!(e.to_string(), "count out of range");
        }
    }

    #[test]
    fn test_token_new_zero_value() {
        let result = Token::new(0);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
        }
    }

    #[test]
    fn test_token_display_trait() {
        let token = Token::new(MIN_VALUE).unwrap();
        let display_str = format!("{token}");
        // Should be a valid string representation
        assert!(!display_str.is_empty());
        assert_eq!(display_str.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn test_token_into_string_conversion() {
        let token = Token::new(MIN_VALUE).unwrap();
        let string_value: String = token.into();
        // Should convert to a valid string
        assert!(!string_value.is_empty());
        assert_eq!(string_value.len(), DEFAULT_LENGTH);
    }

    #[test]
    fn test_token_contains_valid_characters() {
        let token = Token::new(MIN_VALUE).unwrap();
        let token_str = format!("{token}");

        // All characters in the token should be from the CHARS array
        for byte in token_str.as_bytes() {
            assert!(CHARS.contains(byte), "Invalid character found: {}", *byte as char);
        }
    }

    #[test]
    fn test_token_different_values_produce_different_tokens() {
        let token1 = Token::new(MIN_VALUE).unwrap();
        let token2 = Token::new(MIN_VALUE + 1000).unwrap();

        let str1 = format!("{token1}");
        let str2 = format!("{token2}");

        assert_ne!(str1, str2, "Different input values should produce different tokens");
    }

    #[test]
    fn test_token_consistency() {
        let test_value = MIN_VALUE + 12345;
        let token1 = Token::new(test_value).unwrap();
        let token2 = Token::new(test_value).unwrap();

        let str1 = format!("{token1}");
        let str2 = format!("{token2}");

        assert_eq!(str1, str2, "Same input value should produce identical tokens");
    }

    #[test]
    fn test_token_no_null_bytes_in_output() {
        let token = Token::new(MIN_VALUE).unwrap();
        let token_str = format!("{token}");

        // The output should not contain null bytes (which would indicate incomplete conversion)
        assert!(!token_str.contains('\0'), "Token output should not contain null bytes");
    }

    #[test]
    fn test_token_value_array_structure() {
        let token = Token::new(MIN_VALUE).unwrap();

        // Check that the internal array has the expected length
        assert_eq!(token.value.len(), DEFAULT_LENGTH);

        // Check that at least some positions contain valid characters (not all null bytes)
        let has_valid_chars = token.value.iter().any(|&byte| byte != b'\0');
        assert!(has_valid_chars, "Token should contain valid characters, not all null bytes");
    }

    #[test]
    fn test_constants_are_valid() {
        // Test that our constants make sense
        assert!(MIN_VALUE < MAX_VALUE);
        assert!(DEFAULT_LENGTH > 0);
        assert!(BASE > 0);
        assert!(!CHARS.is_empty());
        assert_eq!(CHARS.len(), BASE as usize);
    }

    #[test]
    fn test_edge_case_values() {
        // Test values just within the valid range
        let just_above_min = MIN_VALUE + 1;
        let just_below_max = MAX_VALUE - 1;

        assert!(Token::new(just_above_min).is_ok());
        assert!(Token::new(just_below_max).is_ok());
    }
}
