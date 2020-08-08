use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: i64,
    pub email: String,
}

pub fn encode_token(id: i64, email: String, secret: &str) -> String {
    let claims = Claims {
        exp: 1628380800,
        iat: 1596898542,
        sub: id,
        email: email,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: String, secret: &str) -> Option<Claims> {
    let key = &DecodingKey::from_secret(secret.as_ref());
    let validation = &Validation::default();
    let result = match decode::<Claims>(&token, &key, &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_err) => None,
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_token_returns_token() {
        let secret = "e4d25204-ea68-4307-ae30-1ee4fb39bc9";
        let token = encode_token(123, "test@example.com".to_owned(), secret);
        println!("{}", token);
        assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2MjgzODA4MDAsImlhdCI6MTU5Njg5ODU0Miwic3ViIjoxMjMsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9.xraUIqYvz8mwvLIAmu19r_Xrhf2CvZ-LbfUvL7140D0".to_owned());
    }

    #[test]
    fn decode_token_returns_calims() {
        let secret = "e4d25204-ea68-4307-ae30-1ee4fb39bc9";
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2MjgzODA4MDAsImlhdCI6MTU5Njg5ODU0Miwic3ViIjoxMjMsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9.xraUIqYvz8mwvLIAmu19r_Xrhf2CvZ-LbfUvL7140D0".to_owned();
        let res = decode_token(token, secret).unwrap();
        assert_eq!(res.email, "test@example.com".to_owned());
        assert_eq!(res.sub, 123);
        assert_eq!(res.exp, 1628380800);
        assert_eq!(res.iat, 1596898542);
    }

    #[test]
    fn decode_token_returns_none_when_secret_is_invalid() {
        let secret = "invalid_key";
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2MjgzODA4MDAsImlhdCI6MTU5Njg5ODU0Miwic3ViIjoxMjMsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9.MeX-IGU5TDGxczLehMDbvRMxcf4UL4U6nnQ5NPPrcxA".to_owned();
        let result = decode_token(token, secret);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn decode_token_returns_none_when_token_is_invalid() {
        let secret = "e4d25204-ea68-4307-ae30-1ee4fb39bc9";
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2MjgzODA4MDAsImlhdCI6MTU5Njg5ODU0Miwic3ViIjoxMjMsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9.GBUZXvp1ReH4YxFKang-v5_PejIFdwbMOEcRoBbYSPY".to_owned();
        let result = decode_token(token, secret);
        assert_eq!(result.is_none(), true);
    }
}
