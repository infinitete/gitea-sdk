// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::miscellaneous::*;
use crate::types::{GitignoreTemplateInfo, LabelTemplate, NodeInfo};

pub struct MiscApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerVersion {
    pub version: String,
}

impl<'a> MiscApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

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

    pub async fn get_signing_key_gpg(&self) -> crate::Result<(String, Response)> {
        let (data, response) = self
            .client()
            .get_response(reqwest::Method::GET, "/signing-key.gpg", None, None::<&str>)
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

    pub async fn get_signing_key_ssh(&self) -> crate::Result<(String, Response)> {
        let (data, response) = self
            .client()
            .get_response(reqwest::Method::GET, "/signing-key.pub", None, None::<&str>)
            .await?;
        Ok((String::from_utf8_lossy(&data).to_string(), response))
    }

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
}
