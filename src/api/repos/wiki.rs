// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_wiki.go (6 methods) ──────────────────────────────────

    /// `CreateWikiPage` create a wiki page
    pub async fn create_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateWikiPageOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/wiki/new", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `GetWikiPage` get a wiki page
    pub async fn get_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
    ) -> crate::Result<(WikiPage, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `EditWikiPage` edit a wiki page
    pub async fn edit_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
        opt: CreateWikiPageOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteWikiPage` delete a wiki page
    pub async fn delete_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `ListWikiPages` list wiki pages
    pub async fn list_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
        opt: ListWikiPagesOptions,
    ) -> crate::Result<(Vec<WikiPageMetaData>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/wiki/pages?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetWikiRevisions` get wiki page revisions
    pub async fn get_wiki_revisions(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
        opt: ListWikiPageRevisionsOptions,
    ) -> crate::Result<(WikiCommitList, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/revisions/{}?page={}",
            escaped[0], escaped[1], escaped[2], opt.page
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_create_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/wiki/new"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "SGVsbG8gV29ybGQ=".to_string(),
            message: "create page".to_string(),
        };
        let result = client.repos().create_wiki_page("owner", "repo", opt).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/wiki/new"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "SGVsbG8gV29ybGQ=".to_string(),
            message: "create page".to_string(),
        };
        let result = client.repos().create_wiki_page("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "title": "Home",
                "content_base64": "SGVsbG8gV29ybGQ=",
                "commit_count": 1,
                "sidebar": "",
                "footer": "",
                "html_url": "https://gitea.example.com/owner/repo/wiki/Home",
                "sub_url": "/owner/repo/wiki/Home"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (page, resp) = client
            .repos()
            .get_wiki_page("owner", "repo", "Home")
            .await
            .unwrap();
        assert_eq!(page.title, "Home");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_wiki_page("owner", "repo", "Home").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "VXBkYXRlZCBjb250ZW50".to_string(),
            message: "update page".to_string(),
        };
        let result = client
            .repos()
            .edit_wiki_page("owner", "repo", "Home", opt)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_edit_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "VXBkYXRlZCBjb250ZW50".to_string(),
            message: "update page".to_string(),
        };
        let result = client
            .repos()
            .edit_wiki_page("owner", "repo", "Home", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_wiki_page("owner", "repo", "Home")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_wiki_page("owner", "repo", "Home")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_wiki_pages_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/pages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "title": "Home",
                    "html_url": "https://gitea.example.com/owner/repo/wiki/Home",
                    "sub_url": "/owner/repo/wiki/Home"
                }
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (pages, resp) = client
            .repos()
            .list_wiki_pages("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "Home");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_wiki_pages_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/pages"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_wiki_pages("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_wiki_revisions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/revisions/Home"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "commits": [
                    { "sha": "abc123", "message": "initial page" }
                ]
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (revs, resp) = client
            .repos()
            .get_wiki_revisions("owner", "repo", "Home", Default::default())
            .await
            .unwrap();
        assert_eq!(revs.count, 1);
        assert_eq!(revs.commits.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_wiki_revisions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/revisions/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_wiki_revisions("owner", "repo", "Home", Default::default())
            .await;
        assert!(result.is_err());
    }
}
