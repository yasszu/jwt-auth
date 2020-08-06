use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn get_hash(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    let hex = hasher.result_str();
    hex
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_hash_returns_hex() {
        let hex = get_hash("Hello");
        assert_eq!(hex, "185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969".to_owned());
    }

}