use std::sync::LazyLock;
use domain::{app_error::{AppError, AppResult}, utils::{Claims, JwtManager}};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct SimpleJwtManager;

#[allow(clippy::declare_interior_mutable_const)] // false-positive
const JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    // INFO: okay, so, as I see [`EncodingKey`] doesn't check secret to be
    // valid ascii or what format jwt requires, i don't remember.
    // This is my guess, because secret with spaces generates invalid token (jwt.io displays that).
    std::env::var("JWT_SECRET")
        .map(|s| s.to_ascii_lowercase().replace(" ", "")) // So i hope this will be enough
        .unwrap_or("_do_not_use_this_in_prod_".into())
});

#[allow(clippy::declare_interior_mutable_const)] // false-positive
const JWT_VALIDATION: LazyLock<Validation> = LazyLock::new(|| {
    let mut validation = Validation::default();
    validation.validate_exp = false;
    validation.validate_aud = false;

    validation
});

impl JwtManager for SimpleJwtManager {
    fn serialize(payload: Claims) -> AppResult<String> {
        #[allow(clippy::borrow_interior_mutable_const)] // false-positive
        encode(&Header::default(), &payload, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
            .map_err(|e| AppError::Internal(e.into()))
    }
    
    fn deserialize(token: &str) -> AppResult<Claims> {
        #[allow(clippy::borrow_interior_mutable_const)] // false-positive
        decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &JWT_VALIDATION)
            .map_err(|_| AppError::BadToken)
            .map(|t| t.claims)
    }
}

// FIXME: `JWT_SECRET="anything" carg test` will cause error.
#[cfg(test)]
mod test {
    use domain::utils::{Claims, JwtManager};
    use crate::utils::SimpleJwtManager;

    #[test]
    fn serialize_success() {
        let precomputed_jwtio_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MX0.42u2CTHINAWNG2i5R0UFmdHXrt9EWvxAz8fo8gkxNJk".to_owned();
        let token = SimpleJwtManager::serialize(Claims { sub: 1, exp: 1}).unwrap();

        assert_eq!(precomputed_jwtio_token, token);
    }

    #[test]
    fn deserialize_success() {
        let precomputed_jwtio_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MX0.42u2CTHINAWNG2i5R0UFmdHXrt9EWvxAz8fo8gkxNJk".to_owned();
        let known_claims = Claims { sub: 1, exp: 1 };
        let claims = SimpleJwtManager::deserialize(&precomputed_jwtio_token).unwrap();

        assert_eq!(known_claims.sub, claims.sub);
        assert_eq!(known_claims.exp, claims.exp);
    }

    #[test]
    fn deserialize_fail() {
        let bad_token = "bad.eyJzdWIiOjEsImV4cCI6MX0.token".to_owned();
        let claims = SimpleJwtManager::deserialize(&bad_token);

        assert!(claims.is_err());
    }
}
