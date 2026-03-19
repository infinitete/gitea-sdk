// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Miscellaneous API endpoints for Gitea templates, node info, and signing keys.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::miscellaneous::*;
use crate::types::{
    GitignoreTemplateInfo, LabelTemplate, LicenseTemplateInfo, LicensesTemplateListEntry, NodeInfo,
};

/// API methods for miscellaneous endpoints. Access via [`Client::miscellaneous()`](crate::Client::miscellaneous).
pub struct MiscApi<'a> {
    client: &'a Client,
}

#[derive(Debug, Clone, serde::Deserialize)]
/// Server Version payload type.
pub struct ServerVersion {
    pub version: String,
}

impl<'a> MiscApi<'a> {
    /// Create a new `MiscApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// RenderMarkdown renders a markdown document as HTML
    pub async fn render_markdown(&self, opt: MarkdownOption) -> crate::Result<(String, Response)> {
        let body = json_body(&opt)?;
        let (data, response) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                "/markdown",
                Some(&json_header()),
                Some(body),
            )
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    /// RenderMarkdownRaw renders raw markdown as HTML
    pub async fn render_markdown_raw(&self, markdown: &str) -> crate::Result<(String, Response)> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("text/plain"),
        );
        let (data, response) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                "/markdown/raw",
                Some(&headers),
                Some(markdown.to_string()),
            )
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    /// RenderMarkup renders a markup document as HTML
    pub async fn render_markup(&self, opt: MarkupOption) -> crate::Result<(String, Response)> {
        let body = json_body(&opt)?;
        let (data, response) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                "/markup",
                Some(&json_header()),
                Some(body),
            )
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    /// GetNodeInfo gets the nodeinfo of the Gitea application
    pub async fn get_node_info(&self) -> crate::Result<(NodeInfo, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/nodeinfo",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetSigningKeyGPG gets the default GPG signing key
    pub async fn get_signing_key_gpg(&self) -> crate::Result<(String, Response)> {
        let (data, response) = self
            .client()
            .get_response(reqwest::Method::GET, "/signing-key.gpg", None, None::<&str>)
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    /// GetSigningKeySSH gets the default SSH signing key
    pub async fn get_signing_key_ssh(&self) -> crate::Result<(String, Response)> {
        let (data, response) = self
            .client()
            .get_response(reqwest::Method::GET, "/signing-key.pub", None, None::<&str>)
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    /// ListGitignoresTemplates lists all gitignore templates
    pub async fn list_gitignore_templates(&self) -> crate::Result<(Vec<String>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/gitignore/templates",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetGitignoreTemplateInfo gets information about a gitignore template
    pub async fn get_gitignore_template(
        &self,
        name: &str,
    ) -> crate::Result<(GitignoreTemplateInfo, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[name])?;
        let path = format!("/gitignore/templates/{}", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ListLabelTemplates lists all label templates
    pub async fn list_label_templates(&self) -> crate::Result<(Vec<String>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/label/templates",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetLabelTemplate gets all labels in a template
    pub async fn get_label_template(
        &self,
        name: &str,
    ) -> crate::Result<(Vec<LabelTemplate>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[name])?;
        let path = format!("/label/templates/{}", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ServerVersion returns the version of the server
    pub async fn get_version(&self) -> crate::Result<(String, Response)> {
        let (ver, resp) = self
            .client()
            .get_parsed_response::<ServerVersion, _>(
                reqwest::Method::GET,
                "/version",
                None,
                None::<&str>,
            )
            .await?;
        Ok((ver.version, resp))
    }

    /// ListLicenseTemplates lists the available license templates
    pub async fn list_license_templates(
        &self,
    ) -> crate::Result<(Vec<LicensesTemplateListEntry>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/licenses",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetLicenseTemplateInfo fetches a specific license template
    pub async fn get_license_template(
        &self,
        name: &str,
    ) -> crate::Result<(LicenseTemplateInfo, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[name])?;
        let path = format!("/licenses/{}", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_render_markdown() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markdown"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<h1>Hello</h1>"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = MarkdownOption {
            text: "# Hello".to_string(),
            mode: None,
            context: None,
            wiki: false,
        };
        let (html, resp) = client.miscellaneous().render_markdown(opt).await.unwrap();
        assert_eq!(html, "<h1>Hello</h1>");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_gitignore_templates() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/gitignore/templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec!["Rust", "Go", "Python"]))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (templates, resp) = client
            .miscellaneous()
            .list_gitignore_templates()
            .await
            .unwrap();
        assert_eq!(templates.len(), 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (version, resp) = client.miscellaneous().get_version().await.unwrap();
        assert_eq!(version, "1.22.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_license_templates() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/licenses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"key": "mit", "name": "MIT", "url": "https://example.com/mit"}
            ])))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (templates, resp) = client
            .miscellaneous()
            .list_license_templates()
            .await
            .unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].key, "mit");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_license_template() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/licenses/mit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "key": "mit",
                "name": "MIT",
                "url": "https://example.com/mit",
                "body": "MIT license body",
                "implementation": "Gpl"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (license, resp) = client
            .miscellaneous()
            .get_license_template("mit")
            .await
            .unwrap();
        assert_eq!(license.key, "mit");
        assert_eq!(license.body, "MIT license body");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_render_markdown_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markdown"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = MarkdownOption {
            text: "# Hello".to_string(),
            mode: None,
            context: None,
            wiki: false,
        };
        let result = client.miscellaneous().render_markdown(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_render_markdown_raw() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markdown/raw"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<p>raw markdown</p>"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (html, resp) = client
            .miscellaneous()
            .render_markdown_raw("**bold text**")
            .await
            .unwrap();
        assert_eq!(html, "<p>raw markdown</p>");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_render_markdown_raw_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markdown/raw"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .miscellaneous()
            .render_markdown_raw("**bold text**")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_render_markup() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markup"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<h1>Markup</h1>"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = MarkupOption {
            text: "# Markup".to_string(),
            mode: Some("markdown".to_string()),
            context: None,
            file_path: None,
            wiki: false,
        };
        let (html, resp) = client.miscellaneous().render_markup(opt).await.unwrap();
        assert_eq!(html, "<h1>Markup</h1>");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_render_markup_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/markup"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = MarkupOption {
            text: "# Markup".to_string(),
            mode: None,
            context: None,
            file_path: None,
            wiki: false,
        };
        let result = client.miscellaneous().render_markup(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_node_info() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/nodeinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "2.1",
                "software": {
                    "name": "gitea",
                    "version": "1.22.0",
                    "repository": "https://gitea.com/gitea/gitea",
                    "homepage": "https://gitea.io"
                },
                "protocols": ["activitypub"],
                "services": {
                    "inbound": [],
                    "outbound": []
                },
                "openRegistrations": true,
                "usage": {
                    "users": {
                        "total": 100,
                        "activeHalfyear": 50,
                        "activeMonth": 20
                    },
                    "localPosts": 500,
                    "localComments": 1000
                },
                "metadata": {}
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (info, resp) = client.miscellaneous().get_node_info().await.unwrap();
        assert_eq!(info.version, "2.1");
        assert_eq!(info.software.name, "gitea");
        assert_eq!(info.software.version, "1.22.0");
        assert_eq!(info.usage.users.total, 100);
        assert!(info.open_registrations);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_node_info_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/nodeinfo"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().get_node_info().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_signing_key_gpg() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/signing-key.gpg"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                "-----BEGIN PGP PUBLIC KEY BLOCK-----\n-----END PGP PUBLIC KEY BLOCK-----",
            ))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (key, resp) = client.miscellaneous().get_signing_key_gpg().await.unwrap();
        assert!(key.contains("BEGIN PGP PUBLIC KEY BLOCK"));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_signing_key_gpg_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/signing-key.gpg"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().get_signing_key_gpg().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_signing_key_ssh() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/signing-key.pub"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5 test"),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (key, resp) = client.miscellaneous().get_signing_key_ssh().await.unwrap();
        assert!(key.starts_with("ssh-ed25519"));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_signing_key_ssh_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/signing-key.pub"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().get_signing_key_ssh().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_gitignore_templates_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/gitignore/templates"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().list_gitignore_templates().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_gitignore_template() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/gitignore/templates/Rust"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "Rust",
                "source": "/target/\n**/*.rs.bk\n"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (tmpl, resp) = client
            .miscellaneous()
            .get_gitignore_template("Rust")
            .await
            .unwrap();
        assert_eq!(tmpl.name, "Rust");
        assert!(tmpl.source.contains("/target/"));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_gitignore_template_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/gitignore/templates/Unknown"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .miscellaneous()
            .get_gitignore_template("Unknown")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_label_templates() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/label/templates"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(vec!["Default", "bug", "feature"]),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (templates, resp) = client.miscellaneous().list_label_templates().await.unwrap();
        assert_eq!(templates.len(), 3);
        assert_eq!(templates[0], "Default");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_label_templates_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/label/templates"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().list_label_templates().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_label_template() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/label/templates/Default"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "bug",
                    "color": "ff0000",
                    "description": "Something is broken",
                    "exclusive": false
                },
                {
                    "name": "feature",
                    "color": "00ff00",
                    "description": "New feature request",
                    "exclusive": false
                }
            ])))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (labels, resp) = client
            .miscellaneous()
            .get_label_template("Default")
            .await
            .unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].name, "bug");
        assert_eq!(labels[1].color, "00ff00");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_label_template_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/label/templates/Unknown"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().get_label_template("Unknown").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_version_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().get_version().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_license_templates_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/licenses"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.miscellaneous().list_license_templates().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_license_template_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/licenses/nonexistent"))
            .respond_with(
                ResponseTemplate::new(500).set_body_json(serde_json::json!({"message": "error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .miscellaneous()
            .get_license_template("nonexistent")
            .await;
        assert!(result.is_err());
    }
}
