// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify a Gitea webhook HMAC-SHA256 signature.
///
/// Returns `Ok(true)` if the signature matches, `Ok(false)` if it doesn't,
/// or `Err` if the expected signature is not valid hex.
///
/// Uses constant-time comparison to prevent timing attacks.
pub fn verify_webhook_signature(
    secret: &str,
    expected: &str,
    payload: &[u8],
) -> crate::Result<bool> {
    let expected_bytes = hex::decode(expected)
        .map_err(|_| crate::Error::Validation("invalid hex signature".into()))?;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| crate::Error::Validation("HMAC key error".into()))?;
    mac.update(payload);
    let computed = mac.finalize().into_bytes();

    Ok(constant_time_eq(&computed, &expected_bytes))
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_valid() {
        let secret = "my-secret";
        let payload = b"test-payload";
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload);
        let expected_hex = hex::encode(mac.finalize().into_bytes());

        assert_eq!(
            verify_webhook_signature(secret, &expected_hex, payload).unwrap(),
            true
        );
    }

    #[test]
    fn test_webhook_invalid() {
        let secret = "my-secret";
        let payload = b"test-payload";
        let wrong_sig = "0000000000000000000000000000000000000000000000000000000000000000";

        assert_eq!(
            verify_webhook_signature(secret, wrong_sig, payload).unwrap(),
            false
        );
    }

    #[test]
    fn test_webhook_bad_hex() {
        let result = verify_webhook_signature("secret", "not-hex-at-all!", b"payload");
        assert!(result.is_err());
    }

    #[test]
    fn test_webhook_wrong_length() {
        let secret = "secret";
        let payload = b"payload";
        let wrong_sig = "abcd";
        assert_eq!(
            verify_webhook_signature(secret, wrong_sig, payload).unwrap(),
            false
        );
    }

    #[test]
    fn test_webhook_empty_secret() {
        let secret = "";
        let payload = b"test";
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload);
        let expected_hex = hex::encode(mac.finalize().into_bytes());

        assert_eq!(
            verify_webhook_signature("", &expected_hex, payload).unwrap(),
            true
        );
    }
}
