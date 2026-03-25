use ssh_key::{HashAlg, PrivateKey};

#[must_use]
pub fn ed25519_private_key_bytes() -> &'static [u8] {
    include_bytes!("ssh_fixtures/id_ed25519_test")
}

#[must_use]
pub fn rsa_private_key_bytes() -> &'static [u8] {
    include_bytes!("ssh_fixtures/id_rsa_test")
}

#[must_use]
pub fn rsa_passphrase_private_key_bytes() -> &'static [u8] {
    include_bytes!("ssh_fixtures/id_rsa_passphrase_test")
}

#[must_use]
pub fn rsa_passphrase() -> &'static str {
    "testpassphrase"
}

#[must_use]
pub fn ed25519_public_key_bytes() -> &'static [u8] {
    include_bytes!("ssh_fixtures/id_ed25519_test.pub")
}

#[must_use]
pub fn rsa_public_key_bytes() -> &'static [u8] {
    include_bytes!("ssh_fixtures/id_rsa_test.pub")
}

pub fn ed25519_private_key() -> Result<PrivateKey, ssh_key::Error> {
    PrivateKey::from_openssh(ed25519_private_key_bytes())
}

pub fn rsa_private_key() -> Result<PrivateKey, ssh_key::Error> {
    PrivateKey::from_openssh(rsa_private_key_bytes())
}

pub fn rsa_passphrase_private_key() -> Result<PrivateKey, ssh_key::Error> {
    let key = PrivateKey::from_openssh(rsa_passphrase_private_key_bytes())?;
    key.decrypt(rsa_passphrase())
}

#[test]
fn test_ed25519_key_loads() {
    let key = ed25519_private_key().expect("ED25519 key should load");
    assert!(!key.is_encrypted());
    assert!(!key.fingerprint(HashAlg::Sha256).to_string().is_empty());
}

#[test]
fn test_ed25519_key_algorithm() {
    let key = ed25519_private_key().expect("ED25519 key should load");
    assert_eq!(key.algorithm(), ssh_key::Algorithm::Ed25519);
}

#[test]
fn test_rsa_key_loads() {
    let key = rsa_private_key().expect("RSA key should load");
    assert!(!key.is_encrypted());
    assert!(!key.fingerprint(HashAlg::Sha256).to_string().is_empty());
}

#[test]
fn test_rsa_key_algorithm() {
    let key = rsa_private_key().expect("RSA key should load");
    assert_eq!(key.algorithm(), ssh_key::Algorithm::Rsa { hash: None });
}

#[test]
fn test_rsa_passphrase_key_is_encrypted() {
    let key = PrivateKey::from_openssh(rsa_passphrase_private_key_bytes())
        .expect("RSA passphrase key should parse");
    assert!(key.is_encrypted());
}

#[test]
fn test_rsa_passphrase_key_decrypts() {
    let key = rsa_passphrase_private_key().expect("RSA passphrase key should decrypt");
    assert!(!key.is_encrypted());
    assert!(!key.fingerprint(HashAlg::Sha256).to_string().is_empty());
}

#[test]
fn test_rsa_passphrase_key_wrong_passphrase_fails() {
    let key = PrivateKey::from_openssh(rsa_passphrase_private_key_bytes())
        .expect("RSA passphrase key should parse");
    let result = key.decrypt("wrong-passphrase");
    assert!(result.is_err());
}

#[test]
fn test_ed25519_public_key_loads() {
    let pub_key_str = std::str::from_utf8(ed25519_public_key_bytes())
        .expect("public key file should be valid UTF-8");
    let pub_key =
        ssh_key::PublicKey::from_openssh(pub_key_str).expect("ED25519 public key should load");
    assert_eq!(pub_key.algorithm(), ssh_key::Algorithm::Ed25519);
}

#[test]
fn test_rsa_public_key_loads() {
    let pub_key_str =
        std::str::from_utf8(rsa_public_key_bytes()).expect("public key file should be valid UTF-8");
    let pub_key =
        ssh_key::PublicKey::from_openssh(pub_key_str).expect("RSA public key should load");
    assert_eq!(pub_key.algorithm(), ssh_key::Algorithm::Rsa { hash: None });
}

#[test]
fn test_private_public_key_fingerprints_match() {
    let priv_key = ed25519_private_key().expect("ED25519 key should load");
    let pub_key_str = std::str::from_utf8(ed25519_public_key_bytes())
        .expect("public key file should be valid UTF-8");
    let pub_key =
        ssh_key::PublicKey::from_openssh(pub_key_str).expect("ED25519 public key should load");

    assert_eq!(
        priv_key.public_key().fingerprint(HashAlg::Sha256),
        pub_key.fingerprint(HashAlg::Sha256),
    );
}
