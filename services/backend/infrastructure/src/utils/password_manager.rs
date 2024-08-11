use domain::{app_error::AppResult, utils::PasswordManager};
use rand::{distributions::Alphanumeric, Rng};

pub struct ArgonPasswordManager;

impl PasswordManager for ArgonPasswordManager {
    fn generate_salt() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }
    
    fn hash_password(password: &str, salt: &str) -> AppResult<String> {
        let hash = argon2rs::argon2i_simple(password, salt);

        Ok(String::from_utf8_lossy(&hash).to_string())
    }

    fn verify(password: &str, salt: &str, hash: &str) -> AppResult<bool> {
        let current_hash = Self::hash_password(password, salt)?;

        Ok(argon2rs::verifier::constant_eq(current_hash.as_bytes(), hash.as_bytes()))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use domain::utils::PasswordManager;
    use super::ArgonPasswordManager;

    #[test] // Not actual test, because it's random, but... why not?
    fn argon_n_salts_not_eq() {
        let count = 100;
        let salts: HashSet<String> = (0..100).map(|_| ArgonPasswordManager::generate_salt()).collect();

        assert_eq!(count, salts.len());
    }
    
    #[test]
    fn argon_hash_equality() {
        let salt = "test_salt".to_owned();
        let password = "my_password".to_owned();
        let precomputed_hash = "����<\u{19}\u{7}N���A��9�3[5k\u{13}���\u{10}��٧a�\u{10}".to_owned();
        let hash = ArgonPasswordManager::hash_password(&password, &salt).unwrap();

        assert_eq!(precomputed_hash, hash);
    }
    
    #[test]
    fn argon_verify_success() {
        let salt = "test_salt".to_owned();
        let password = "my_password".to_owned();
        let hash = ArgonPasswordManager::hash_password(&password, &salt).unwrap();
        let is_success = ArgonPasswordManager::verify(&password, &salt, &hash).unwrap();

        assert!(is_success);
    }

    #[test]
    fn argon_verify_fail() {
        let salt = "test_salt".to_owned();
        let password = "my_password".to_owned();
        let hash = ArgonPasswordManager::hash_password(&password, &salt).unwrap();
        let is_success = ArgonPasswordManager::verify(&password, "different_salt", &hash).unwrap();

        assert!(!is_success);
    }

    #[test]
    fn argon_fullfeatured() {
        let password = "super_hard_password".to_owned();
        let salt = ArgonPasswordManager::generate_salt();
        let hash = ArgonPasswordManager::hash_password(&password, &salt).unwrap();
        let is_success = ArgonPasswordManager::verify(&password, &salt, &hash).unwrap();

        assert!(is_success);
    }
}
