// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Windows Pageant / OpenSSH agent communication module.
//!
//! This module provides SSH agent protocol message encoding/decoding that
//! works on any platform, and platform-specific connection functions:
//!
//! - On **Windows**, connects to Pageant via shared memory / window messages
//!   or to OpenSSH agent via named pipe (`\\.\pipe\openssh-agent`).
//! - On **non-Windows**, the connection functions return an error since
//!   Unix SSH agent communication is handled by [`super::httpsig`].

#![allow(dead_code)]

/// SSH agent request message type: list identities.
pub const SSH_AGENT_REQUEST_IDENTITIES: u8 = 11;

/// SSH agent response message type: identities answer.
pub const SSH_AGENT_IDENTITIES_ANSWER: u8 = 12;

/// SSH agent request message type: sign data.
pub const SSH_AGENT_SIGN_REQUEST: u8 = 13;

/// SSH agent response message type: sign response.
pub const SSH_AGENT_SIGN_RESPONSE: u8 = 14;

/// Named pipe path for OpenSSH agent on Windows.
#[cfg(windows)]
const OPENSSH_AGENT_PIPE: &str = r"\\.\pipe\openssh-agent";

/// Named pipe path for Pageant agent on Windows.
#[cfg(windows)]
const PAGEANT_PIPE: &str = r"\\.\pipe\pageant";

// ---------------------------------------------------------------------------
// Platform-independent protocol encoding / decoding
// ---------------------------------------------------------------------------

/// Build a complete SSH agent wire message: 4-byte big-endian length prefix
/// followed by the payload (message type byte + body).
pub fn encode_message(msg_type: u8, body: &[u8]) -> Vec<u8> {
    let len = (1 + body.len()) as u32;
    let mut buf = Vec::with_capacity(4 + len as usize);
    buf.extend_from_slice(&len.to_be_bytes());
    buf.push(msg_type);
    buf.extend_from_slice(body);
    buf
}

/// Decode the 4-byte big-endian length prefix from a raw agent response.
///
/// Returns the length value and the remaining payload slice.
pub fn decode_length_prefix(data: &[u8]) -> crate::Result<(u32, &[u8])> {
    if data.len() < 4 {
        return Err(crate::Error::SshSign(
            "response too short for length prefix".into(),
        ));
    }
    let len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let payload = &data[4..];
    if payload.len() < len as usize {
        return Err(crate::Error::SshSign(format!(
            "expected {} payload bytes, got {}",
            len,
            payload.len()
        )));
    }
    Ok((len, &payload[..len as usize]))
}

/// Encode an SSH agent identities request message.
///
/// Wire format: `<4-byte length><0x0B>` (body is empty).
pub fn encode_request_identities() -> Vec<u8> {
    encode_message(SSH_AGENT_REQUEST_IDENTITIES, &[])
}

/// Decode an SSH agent identities answer message.
///
/// Wire format: `<4-byte length><0x0C><4-byte key-count>[<4-byte blob-len><blob><4-byte comment-len><comment>]...`
///
/// Returns a list of `(key_blob, comment)` pairs.
pub fn decode_identities_answer(data: &[u8]) -> crate::Result<Vec<(Vec<u8>, String)>> {
    let (_len, payload) = decode_length_prefix(data)?;

    if payload.is_empty() {
        return Err(crate::Error::SshSign(
            "identities answer payload is empty".into(),
        ));
    }

    let msg_type = payload[0];
    if msg_type != SSH_AGENT_IDENTITIES_ANSWER {
        return Err(crate::Error::SshSign(format!(
            "expected identities answer (12), got {}",
            msg_type
        )));
    }

    let mut pos = 1usize;

    // Key count
    if pos + 4 > payload.len() {
        return Err(crate::Error::SshSign(
            "identities answer truncated at key count".into(),
        ));
    }
    let key_count = u32::from_be_bytes([
        payload[pos],
        payload[pos + 1],
        payload[pos + 2],
        payload[pos + 3],
    ]) as usize;
    pos += 4;

    let mut identities = Vec::with_capacity(key_count);

    for _ in 0..key_count {
        // Key blob
        if pos + 4 > payload.len() {
            return Err(crate::Error::SshSign(
                "identities answer truncated at key blob length".into(),
            ));
        }
        let blob_len = u32::from_be_bytes([
            payload[pos],
            payload[pos + 1],
            payload[pos + 2],
            payload[pos + 3],
        ]) as usize;
        pos += 4;

        if pos + blob_len > payload.len() {
            return Err(crate::Error::SshSign(format!(
                "identities answer truncated: key blob expects {} bytes, {} available",
                blob_len,
                payload.len() - pos
            )));
        }
        let blob = payload[pos..pos + blob_len].to_vec();
        pos += blob_len;

        // Comment
        if pos + 4 > payload.len() {
            return Err(crate::Error::SshSign(
                "identities answer truncated at comment length".into(),
            ));
        }
        let comment_len = u32::from_be_bytes([
            payload[pos],
            payload[pos + 1],
            payload[pos + 2],
            payload[pos + 3],
        ]) as usize;
        pos += 4;

        if pos + comment_len > payload.len() {
            return Err(crate::Error::SshSign(format!(
                "identities answer truncated: comment expects {} bytes, {} available",
                comment_len,
                payload.len() - pos
            )));
        }
        let comment_bytes = &payload[pos..pos + comment_len];
        let comment = String::from_utf8_lossy(comment_bytes).into_owned();
        pos += comment_len;

        identities.push((blob, comment));
    }

    Ok(identities)
}

/// Encode an SSH agent sign request message.
///
/// Wire format: `<4-byte length><0x0D><4-byte key-blob-len><key-blob><4-byte data-len><data><4-byte flags>`.
pub fn encode_sign_request(key_blob: &[u8], data: &[u8], flags: u32) -> Vec<u8> {
    // Body: key_blob_len(4) + key_blob + data_len(4) + data + flags(4)
    let mut body = Vec::with_capacity(4 + key_blob.len() + 4 + data.len() + 4);
    body.extend_from_slice(&(key_blob.len() as u32).to_be_bytes());
    body.extend_from_slice(key_blob);
    body.extend_from_slice(&(data.len() as u32).to_be_bytes());
    body.extend_from_slice(data);
    body.extend_from_slice(&flags.to_be_bytes());
    encode_message(SSH_AGENT_SIGN_REQUEST, &body)
}

/// Decode an SSH agent sign response message.
///
/// Wire format: `<4-byte length><0x0E><4-byte sig-blob-len><sig-blob>`.
///
/// Returns the raw signature blob.
pub fn decode_sign_response(data: &[u8]) -> crate::Result<Vec<u8>> {
    let (_len, payload) = decode_length_prefix(data)?;

    if payload.is_empty() {
        return Err(crate::Error::SshSign(
            "sign response payload is empty".into(),
        ));
    }

    let msg_type = payload[0];
    if msg_type != SSH_AGENT_SIGN_RESPONSE {
        return Err(crate::Error::SshSign(format!(
            "expected sign response (14), got {}",
            msg_type
        )));
    }

    let mut pos = 1usize;

    if pos + 4 > payload.len() {
        return Err(crate::Error::SshSign(
            "sign response truncated at signature length".into(),
        ));
    }
    let sig_len = u32::from_be_bytes([
        payload[pos],
        payload[pos + 1],
        payload[pos + 2],
        payload[pos + 3],
    ]) as usize;
    pos += 4;

    if pos + sig_len > payload.len() {
        return Err(crate::Error::SshSign(format!(
            "sign response truncated: signature expects {} bytes, {} available",
            sig_len,
            payload.len() - pos
        )));
    }

    Ok(payload[pos..pos + sig_len].to_vec())
}

// ---------------------------------------------------------------------------
// Platform-specific connection helpers
// ---------------------------------------------------------------------------

/// Connect to the Windows SSH agent and list available identities.
///
/// Attempts the OpenSSH named pipe first, then Pageant.
#[cfg(windows)]
pub async fn list_identities() -> crate::Result<Vec<(Vec<u8>, String)>> {
    match list_identities_openssh_pipe().await {
        Ok(identities) => return Ok(identities),
        Err(e) => tracing::debug!("OpenSSH pipe failed: {e}, trying Pageant"),
    }
    list_identities_pageant().await
}

/// Connect to the Windows SSH agent and sign data with a specific key.
///
/// Attempts the OpenSSH named pipe first, then Pageant.
#[cfg(windows)]
pub async fn sign_data(key_blob: &[u8], data: &[u8], flags: u32) -> crate::Result<Vec<u8>> {
    match sign_data_openssh_pipe(key_blob, data, flags).await {
        Ok(sig) => return Ok(sig),
        Err(e) => tracing::debug!("OpenSSH pipe failed: {e}, trying Pageant"),
    }
    sign_data_pageant(key_blob, data, flags).await
}

/// Connect to the OpenSSH Windows agent via named pipe and list identities.
#[cfg(windows)]
async fn list_identities_openssh_pipe() -> crate::Result<Vec<(Vec<u8>, String)>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::windows::named_pipe::ClientOptions;

    let pipe = ClientOptions::new()
        .open(OPENSSH_AGENT_PIPE)
        .map_err(|e| crate::Error::SshSign(format!("failed to open OpenSSH agent pipe: {e}")))?;

    let msg = encode_request_identities();
    agent_roundtrip(&pipe, &msg).await
}

/// Connect to the OpenSSH Windows agent via named pipe and sign data.
#[cfg(windows)]
async fn sign_data_openssh_pipe(
    key_blob: &[u8],
    data: &[u8],
    flags: u32,
) -> crate::Result<Vec<u8>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::windows::named_pipe::ClientOptions;

    let pipe = ClientOptions::new()
        .open(OPENSSH_AGENT_PIPE)
        .map_err(|e| crate::Error::SshSign(format!("failed to open OpenSSH agent pipe: {e}")))?;

    let msg = encode_sign_request(key_blob, data, flags);
    let response = agent_roundtrip_raw(&pipe, &msg).await?;
    decode_sign_response(&response)
}

/// Perform a named-pipe roundtrip: write a message, read the full response.
#[cfg(windows)]
async fn agent_roundtrip_raw(
    pipe: &tokio::net::windows::named_pipe::NamedPipeClient,
    msg: &[u8],
) -> crate::Result<Vec<u8>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut pipe = pipe
        .try_clone()
        .map_err(|e| crate::Error::SshSign(format!("failed to clone pipe handle: {e}")))?;

    pipe.write_all(msg)
        .await
        .map_err(|e| crate::Error::SshSign(format!("failed to write to agent pipe: {e}")))?;

    // Read the 4-byte length prefix first
    let mut len_buf = [0u8; 4];
    pipe.read_exact(&mut len_buf)
        .await
        .map_err(|e| crate::Error::SshSign(format!("failed to read agent response length: {e}")))?;
    let resp_len = u32::from_be_bytes(len_buf) as usize;

    // Cap response size to prevent unbounded allocation
    const MAX_RESPONSE_SIZE: usize = 256 * 1024;
    if resp_len > MAX_RESPONSE_SIZE {
        return Err(crate::Error::SshSign(format!(
            "agent response too large: {} bytes (max {})",
            resp_len, MAX_RESPONSE_SIZE
        )));
    }

    let mut resp_buf = vec![0u8; resp_len];
    pipe.read_exact(&mut resp_buf)
        .await
        .map_err(|e| crate::Error::SshSign(format!("failed to read agent response body: {e}")))?;

    // Prepend the length prefix for decode functions
    let mut full_response = Vec::with_capacity(4 + resp_len);
    full_response.extend_from_slice(&len_buf);
    full_response.extend_from_slice(&resp_buf);
    Ok(full_response)
}

/// Perform a named-pipe roundtrip and decode as identities answer.
#[cfg(windows)]
async fn agent_roundtrip(
    pipe: &tokio::net::windows::named_pipe::NamedPipeClient,
    msg: &[u8],
) -> crate::Result<Vec<(Vec<u8>, String)>> {
    let response = agent_roundtrip_raw(pipe, msg).await?;
    decode_identities_answer(&response)
}

// ---------------------------------------------------------------------------
// Pageant-specific protocol (Windows shared memory + window messages)
// ---------------------------------------------------------------------------

/// List identities via Pageant's shared memory + window message protocol.
///
/// Pageant does not use named pipes. Instead:
/// 1. Open the `Pageant` file mapping (shared memory).
/// 2. Write the request into the shared memory.
/// 3. Send a `WM_COPYDATA` window message to the Pageant window.
/// 4. Read the response from the shared memory.
///
/// This is a placeholder that documents the protocol. Full implementation
/// requires `windows` crate for Win32 API access.
#[cfg(windows)]
async fn list_identities_pageant() -> crate::Result<Vec<(Vec<u8>, String)>> {
    Err(crate::Error::SshSign(
        "Pageant shared-memory protocol not yet implemented".into(),
    ))
}

/// Sign data via Pageant's shared memory + window message protocol.
///
/// See [`list_identities_pageant`] for protocol description.
#[cfg(windows)]
async fn sign_data_pageant(_key_blob: &[u8], _data: &[u8], _flags: u32) -> crate::Result<Vec<u8>> {
    Err(crate::Error::SshSign(
        "Pageant shared-memory protocol not yet implemented".into(),
    ))
}

// ---------------------------------------------------------------------------
// Non-Windows stubs
// ---------------------------------------------------------------------------

/// Not available on non-Windows platforms.
#[cfg(not(windows))]
pub async fn list_identities() -> crate::Result<Vec<(Vec<u8>, String)>> {
    Err(crate::Error::SshSign(
        "SSH agent identity listing is not supported on this platform".into(),
    ))
}

/// Not available on non-Windows platforms.
#[cfg(not(windows))]
pub async fn sign_data(_key_blob: &[u8], _data: &[u8], _flags: u32) -> crate::Result<Vec<u8>> {
    Err(crate::Error::SshSign(
        "SSH agent signing is not supported on this platform".into(),
    ))
}

// ---------------------------------------------------------------------------
// Tests (platform-independent protocol tests)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // encode_message
    // -----------------------------------------------------------------------

    #[test]
    fn test_encode_message_type_only() {
        let msg = encode_message(11, &[]);
        // length=1, type=11
        assert_eq!(msg, vec![0, 0, 0, 1, 11]);
    }

    #[test]
    fn test_encode_message_with_body() {
        let body = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let msg = encode_message(13, &body);
        // length=5 (1 type + 4 body), type=13, then body
        assert_eq!(msg, vec![0, 0, 0, 5, 13, 0xDE, 0xAD, 0xBE, 0xEF]);
    }

    // -----------------------------------------------------------------------
    // decode_length_prefix
    // -----------------------------------------------------------------------

    #[test]
    fn test_decode_length_prefix_valid() {
        let data = vec![0, 0, 0, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (len, payload) = decode_length_prefix(&data).expect("decode should succeed");
        assert_eq!(len, 9);
        assert_eq!(payload, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_decode_length_prefix_truncated_header() {
        let data = vec![0, 0, 1];
        let result = decode_length_prefix(&data);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("too short"));
    }

    #[test]
    fn test_decode_length_prefix_truncated_payload() {
        // Claims 100 bytes but only has 2
        let data = vec![0, 0, 0, 100, 0xAA, 0xBB];
        let result = decode_length_prefix(&data);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("expected 100 payload bytes"));
    }

    // -----------------------------------------------------------------------
    // encode_request_identities
    // -----------------------------------------------------------------------

    #[test]
    fn test_encode_request_identities() {
        let msg = encode_request_identities();
        assert_eq!(msg, vec![0, 0, 0, 1, SSH_AGENT_REQUEST_IDENTITIES]);
    }

    // -----------------------------------------------------------------------
    // decode_identities_answer
    // -----------------------------------------------------------------------

    #[test]
    fn test_decode_identities_answer_empty() {
        // 0 keys
        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &[0, 0, 0, 0]);
        let identities = decode_identities_answer(&data).expect("decode should succeed");
        assert!(identities.is_empty());
    }

    #[test]
    fn test_decode_identities_answer_one_key() {
        let key_blob = vec![0x01, 0x02, 0x03, 0x04];
        let comment = b"test-key";

        let mut body = Vec::new();
        // key count = 1
        body.extend_from_slice(&1u32.to_be_bytes());
        // key blob length + blob
        body.extend_from_slice(&(key_blob.len() as u32).to_be_bytes());
        body.extend_from_slice(&key_blob);
        // comment length + comment
        body.extend_from_slice(&(comment.len() as u32).to_be_bytes());
        body.extend_from_slice(comment);

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let identities = decode_identities_answer(&data).expect("decode should succeed");
        assert_eq!(identities.len(), 1);
        assert_eq!(identities[0].0, key_blob);
        assert_eq!(identities[0].1, "test-key");
    }

    #[test]
    fn test_decode_identities_answer_two_keys() {
        let key_blob1 = vec![0x01];
        let comment1 = b"first";
        let key_blob2 = vec![0x02, 0x03];
        let comment2 = b"second";

        let mut body = Vec::new();
        body.extend_from_slice(&2u32.to_be_bytes());

        body.extend_from_slice(&(key_blob1.len() as u32).to_be_bytes());
        body.extend_from_slice(&key_blob1);
        body.extend_from_slice(&(comment1.len() as u32).to_be_bytes());
        body.extend_from_slice(comment1);

        body.extend_from_slice(&(key_blob2.len() as u32).to_be_bytes());
        body.extend_from_slice(&key_blob2);
        body.extend_from_slice(&(comment2.len() as u32).to_be_bytes());
        body.extend_from_slice(comment2);

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let identities = decode_identities_answer(&data).expect("decode should succeed");
        assert_eq!(identities.len(), 2);
        assert_eq!(identities[0].0, key_blob1);
        assert_eq!(identities[0].1, "first");
        assert_eq!(identities[1].0, key_blob2);
        assert_eq!(identities[1].1, "second");
    }

    #[test]
    fn test_decode_identities_answer_empty_comment() {
        let key_blob = vec![0xAA];
        let comment: &[u8] = b"";

        let mut body = Vec::new();
        body.extend_from_slice(&1u32.to_be_bytes());
        body.extend_from_slice(&(key_blob.len() as u32).to_be_bytes());
        body.extend_from_slice(&key_blob);
        body.extend_from_slice(&(comment.len() as u32).to_be_bytes());
        body.extend_from_slice(comment);

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let identities = decode_identities_answer(&data).expect("decode should succeed");
        assert_eq!(identities.len(), 1);
        assert_eq!(identities[0].1, "");
    }

    #[test]
    fn test_decode_identities_answer_wrong_type() {
        // Type 99 instead of 12
        let data = encode_message(99, &[0, 0, 0, 0]);
        let result = decode_identities_answer(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected identities answer (12)")
        );
    }

    #[test]
    fn test_decode_identities_answer_truncated_at_count() {
        // Length says 1 byte but no count bytes
        let data = vec![0, 0, 0, 1, SSH_AGENT_IDENTITIES_ANSWER];
        let result = decode_identities_answer(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("truncated at key count")
        );
    }

    #[test]
    fn test_decode_identities_answer_truncated_blob() {
        let mut body = Vec::new();
        body.extend_from_slice(&1u32.to_be_bytes()); // 1 key
        body.extend_from_slice(&100u32.to_be_bytes()); // claims 100-byte blob
        // but no actual blob data

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let result = decode_identities_answer(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("truncated: key blob expects")
        );
    }

    #[test]
    fn test_decode_identities_answer_truncated_comment() {
        let key_blob = vec![0x01];
        let mut body = Vec::new();
        body.extend_from_slice(&1u32.to_be_bytes());
        body.extend_from_slice(&(key_blob.len() as u32).to_be_bytes());
        body.extend_from_slice(&key_blob);
        body.extend_from_slice(&50u32.to_be_bytes()); // claims 50-byte comment
        // but no comment data

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let result = decode_identities_answer(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("truncated: comment expects")
        );
    }

    #[test]
    fn test_decode_identities_answer_empty_payload() {
        // Empty data can't even have a length prefix
        let result = decode_identities_answer(&[]);
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // encode_sign_request
    // -----------------------------------------------------------------------

    #[test]
    fn test_encode_sign_request() {
        let key_blob = vec![0x01, 0x02];
        let data = vec![0xAA, 0xBB, 0xCC];
        let flags = 0;

        let msg = encode_sign_request(&key_blob, &data, flags);

        // Decode to verify structure
        assert_eq!(&msg[0..4], &[0, 0, 0, 1 + 4 + 2 + 4 + 3 + 4]); // length
        assert_eq!(msg[4], SSH_AGENT_SIGN_REQUEST);

        // Body: key_blob_len(4) + key_blob(2) + data_len(4) + data(3) + flags(4) = 17
        let body = &msg[5..];
        assert_eq!(&body[0..4], &(2u32.to_be_bytes())); // key blob len
        assert_eq!(&body[4..6], &[0x01, 0x02]); // key blob
        assert_eq!(&body[6..10], &(3u32.to_be_bytes())); // data len
        assert_eq!(&body[10..13], &[0xAA, 0xBB, 0xCC]); // data
        assert_eq!(&body[13..17], &0u32.to_be_bytes()); // flags
    }

    #[test]
    fn test_encode_sign_request_with_flags() {
        let key_blob = vec![0x42];
        let data = vec![0xFF];
        let flags = 2; // SSH_AGENT_RSA_SHA2_256

        let msg = encode_sign_request(&key_blob, &data, flags);

        let body = &msg[5..];
        assert_eq!(&body[0..4], &(1u32.to_be_bytes()));
        assert_eq!(&body[4..5], &[0x42]);
        assert_eq!(&body[5..9], &(1u32.to_be_bytes()));
        assert_eq!(&body[9..10], &[0xFF]);
        assert_eq!(&body[10..14], &2u32.to_be_bytes());
    }

    // -----------------------------------------------------------------------
    // decode_sign_response
    // -----------------------------------------------------------------------

    #[test]
    fn test_decode_sign_response_valid() {
        let sig_blob = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut body = Vec::new();
        body.extend_from_slice(&(sig_blob.len() as u32).to_be_bytes());
        body.extend_from_slice(&sig_blob);

        let data = encode_message(SSH_AGENT_SIGN_RESPONSE, &body);
        let sig = decode_sign_response(&data).expect("decode should succeed");
        assert_eq!(sig, sig_blob);
    }

    #[test]
    fn test_decode_sign_response_empty_sig() {
        let body = vec![0, 0, 0, 0]; // 0-length signature
        let data = encode_message(SSH_AGENT_SIGN_RESPONSE, &body);
        let sig = decode_sign_response(&data).expect("decode should succeed");
        assert!(sig.is_empty());
    }

    #[test]
    fn test_decode_sign_response_wrong_type() {
        let body = vec![0, 0, 0, 0];
        let data = encode_message(42, &body);
        let result = decode_sign_response(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("expected sign response (14)")
        );
    }

    #[test]
    fn test_decode_sign_response_truncated_length() {
        // Type byte only, no sig length
        let data = encode_message(SSH_AGENT_SIGN_RESPONSE, &[]);
        let result = decode_sign_response(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("truncated at signature length")
        );
    }

    #[test]
    fn test_decode_sign_response_truncated_sig() {
        let mut body = Vec::new();
        body.extend_from_slice(&10u32.to_be_bytes()); // claims 10 bytes
        body.extend_from_slice(&[0x01, 0x02]); // but only 2

        let data = encode_message(SSH_AGENT_SIGN_RESPONSE, &body);
        let result = decode_sign_response(&data);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("truncated: signature expects")
        );
    }

    #[test]
    fn test_decode_sign_response_empty_payload() {
        let result = decode_sign_response(&[]);
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Round-trip tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_roundtrip_identities() {
        // Build a realistic-ish identities answer with 2 keys
        let key1 = vec![
            0x00, 0x00, 0x00, 0x0B, b's', b's', b'h', b'-', b'e', b'd', b'2', b'5', b'5', b'1',
            b'9',
        ];
        let comment1 = b"alice@work";
        let key2 = vec![
            0x00, 0x00, 0x00, 0x09, b's', b's', b'h', b'-', b'r', b's', b'a', b' ', b' ',
        ];
        let comment2 = b"bob@home";

        let mut body = Vec::new();
        body.extend_from_slice(&2u32.to_be_bytes());

        body.extend_from_slice(&(key1.len() as u32).to_be_bytes());
        body.extend_from_slice(&key1);
        body.extend_from_slice(&(comment1.len() as u32).to_be_bytes());
        body.extend_from_slice(comment1);

        body.extend_from_slice(&(key2.len() as u32).to_be_bytes());
        body.extend_from_slice(&key2);
        body.extend_from_slice(&(comment2.len() as u32).to_be_bytes());
        body.extend_from_slice(comment2);

        let data = encode_message(SSH_AGENT_IDENTITIES_ANSWER, &body);
        let identities = decode_identities_answer(&data).expect("roundtrip should succeed");

        assert_eq!(identities.len(), 2);
        assert_eq!(identities[0].0, key1);
        assert_eq!(identities[0].1, "alice@work");
        assert_eq!(identities[1].0, key2);
        assert_eq!(identities[1].1, "bob@home");
    }

    #[test]
    fn test_roundtrip_sign() {
        let sig_blob = vec![0x00, 0x00, 0x00, 0x40, 0xDE, 0xAD];

        let mut body = Vec::new();
        body.extend_from_slice(&(sig_blob.len() as u32).to_be_bytes());
        body.extend_from_slice(&sig_blob);

        let data = encode_message(SSH_AGENT_SIGN_RESPONSE, &body);
        let sig = decode_sign_response(&data).expect("roundtrip should succeed");
        assert_eq!(sig, sig_blob);
    }

    // -----------------------------------------------------------------------
    // Non-Windows stubs
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_list_identities_not_windows() {
        // On Linux, this should return an error
        let result = list_identities().await;
        #[cfg(not(windows))]
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sign_data_not_windows() {
        let result = sign_data(&[1, 2, 3], &[4, 5, 6], 0).await;
        #[cfg(not(windows))]
        assert!(result.is_err());
    }
}
