// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! HTTP signature authentication for the Gitea API.

#[allow(dead_code)]
pub(crate) struct SignedStringComponents {
    /// HTTP method, lowercased (e.g. "post", "get").
    pub(crate) method: String,
    /// Request path including query string (e.g. "/api/v1/repos/owner/repo").
    pub(crate) path: String,
    /// Unix timestamp when the signature was created.
    pub(crate) created: i64,
    /// Unix timestamp when the signature expires.
    pub(crate) expires: i64,
    /// Optional Digest header value (e.g. "SHA256=abc123=").
    pub(crate) digest: Option<String>,
    /// Optional extra headers to include in the signed headers list and signed string.
    /// Used for `x-ssh-certificate` in certificate mode.
    pub(crate) extra_headers: Vec<(String, String)>,
}

#[allow(dead_code)]
impl SignedStringComponents {
    /// Returns the list of signed header names for this request.
    ///
    /// Always includes `(request-target)`, `(created)`, and `(expires)`.
    /// When a digest is present, also includes `digest`.
    /// When extra headers are present, they are appended.
    pub(crate) fn headers_list(&self) -> String {
        let mut parts: Vec<&str> = vec!["(request-target)", "(created)", "(expires)"];
        if self.digest.is_some() {
            parts.push("digest");
        }
        if !self.extra_headers.is_empty() {
            let extra_names: Vec<&str> =
                self.extra_headers.iter().map(|(n, _)| n.as_str()).collect();
            parts.extend(extra_names);
        }
        parts.join(" ")
    }

    /// Builds the signed string from the request components.
    ///
    /// Each line is `header_name: header_value` in the standard order.
    /// The method in `(request-target)` is lowercased per the spec.
    pub(crate) fn build(&self) -> crate::Result<String> {
        if self.method.is_empty() {
            return Err(crate::Error::Validation(
                "HTTP method must not be empty".into(),
            ));
        }
        if self.path.is_empty() {
            return Err(crate::Error::Validation(
                "request path must not be empty".into(),
            ));
        }

        let mut lines = Vec::new();
        lines.push(format!(
            "(request-target): {} {}",
            self.method.to_lowercase(),
            self.path
        ));
        lines.push(format!("(created): {}", self.created));
        lines.push(format!("(expires): {}", self.expires));

        if let Some(ref digest) = self.digest {
            lines.push(format!("digest: {}", digest));
        }

        for (name, value) in &self.extra_headers {
            lines.push(format!("{name}: {value}"));
        }

        Ok(lines.join("\n"))
    }
}

#[allow(dead_code)]
pub(crate) struct HttpSignature {
    /// The SSH key identifier, e.g. `ssh-ed25519 SHA256:abc123=`.
    pub(crate) key_id: String,
    /// The signing algorithm, e.g. `ed25519` or `rsa-sha2-256`.
    pub(crate) algorithm: String,
    /// Space-separated list of signed header names.
    pub(crate) headers: String,
    /// Base64-encoded signature bytes.
    pub(crate) signature: String,
}

#[allow(dead_code)]
impl HttpSignature {
    /// Formats this signature as a modern `Signature` header value.
    ///
    /// Returns the header value only (without the `Signature: ` prefix),
    /// e.g. `keyId="...",algorithm="...",headers="...",signature="..."`.
    pub(crate) fn to_signature_header(&self) -> String {
        format!(
            r#"keyId="{}",algorithm="{}",headers="{}",signature="{}""#,
            self.key_id, self.algorithm, self.headers, self.signature
        )
    }

    /// Formats this signature as a legacy `Authorization: Signature` header value.
    ///
    /// Returns the full header value including the `Signature` prefix,
    /// e.g. `Signature keyId="...",algorithm="...",headers="...",signature="..."`.
    pub(crate) fn to_authorization_header(&self) -> String {
        format!("Signature {}", self.to_signature_header())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signed_string_construction() {
        let components = crate::auth::httpsig::SignedStringComponents {
            method: "POST".to_string(),
            path: "/api/v1/repos/owner/repo".to_string(),
            created: 1704067200,
            expires: 1704067210,
            digest: Some("SHA256=abc123=".to_string()),
            extra_headers: vec![],
        };

        let signed_string = components.build().unwrap();
        let expected = [
            "(request-target): post /api/v1/repos/owner/repo",
            "(created): 1704067200",
            "(expires): 1704067210",
            "digest: SHA256=abc123=",
        ]
        .join("\n");

        assert_eq!(signed_string, expected);
    }

    #[test]
    fn test_signed_string_without_digest() {
        let components = crate::auth::httpsig::SignedStringComponents {
            method: "GET".to_string(),
            path: "/api/v1/user".to_string(),
            created: 1704153600,
            expires: 1704153610,
            digest: None,
            extra_headers: vec![],
        };

        let signed_string = components.build().unwrap();
        let expected = [
            "(request-target): get /api/v1/user",
            "(created): 1704153600",
            "(expires): 1704153610",
        ]
        .join("\n");

        assert_eq!(signed_string, expected);
    }

    #[test]
    fn test_modern_signature_header() {
        let sig = crate::auth::httpsig::HttpSignature {
            key_id: "SHA256:abc123=".to_string(),
            algorithm: "ed25519".to_string(),
            headers: "(request-target) (created) (expires) digest".to_string(),
            signature: "Zm9vYmFyYmF6".to_string(),
        };

        let header = sig.to_signature_header();
        let expected = r#"keyId="SHA256:abc123=",algorithm="ed25519",headers="(request-target) (created) (expires) digest",signature="Zm9vYmFyYmF6""#;

        assert_eq!(header, expected);
    }

    #[test]
    fn test_legacy_authorization_header() {
        let sig = crate::auth::httpsig::HttpSignature {
            key_id: "SHA256:xyz789=".to_string(),
            algorithm: "rsa-sha2-256".to_string(),
            headers: "(request-target) (created) (expires)".to_string(),
            signature: "c2lnbmF0dXJl".to_string(),
        };

        let header = sig.to_authorization_header();
        let expected = r#"Signature keyId="SHA256:xyz789=",algorithm="rsa-sha2-256",headers="(request-target) (created) (expires)",signature="c2lnbmF0dXJl""#;

        assert_eq!(header, expected);
    }

    #[test]
    fn test_signed_string_headers_list_with_digest() {
        let components = crate::auth::httpsig::SignedStringComponents {
            method: "POST".to_string(),
            path: "/api/v1/repos/owner/repo".to_string(),
            created: 1704067200,
            expires: 1704067210,
            digest: Some("SHA256=abc123=".to_string()),
            extra_headers: vec![],
        };

        let headers_list = components.headers_list();
        assert_eq!(headers_list, "(request-target) (created) (expires) digest");
    }

    #[test]
    fn test_signed_string_headers_list_without_digest() {
        let components = crate::auth::httpsig::SignedStringComponents {
            method: "GET".to_string(),
            path: "/api/v1/user".to_string(),
            created: 1704067200,
            expires: 1704067210,
            digest: None,
            extra_headers: vec![],
        };

        let headers_list = components.headers_list();
        assert_eq!(headers_list, "(request-target) (created) (expires)");
    }

    #[test]
    fn test_signed_string_with_extra_headers() {
        let components = crate::auth::httpsig::SignedStringComponents {
            method: "POST".to_string(),
            path: "/api/v1/repos/owner/repo".to_string(),
            created: 1704067200,
            expires: 1704067210,
            digest: None,
            extra_headers: vec![("x-ssh-certificate".to_string(), "certdata".to_string())],
        };

        let headers_list = components.headers_list();
        assert_eq!(
            headers_list,
            "(request-target) (created) (expires) x-ssh-certificate"
        );

        let signed_string = components.build().unwrap();
        assert!(signed_string.contains("x-ssh-certificate: certdata"));
    }
}
