// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Package API endpoints for managing Gitea packages and container registry.

use crate::Client;
use crate::Response;
use crate::options::package::ListPackagesOptions;
use crate::pagination::QueryEncode;
use crate::types::{Package, PackageFile};

/// API methods for packages. Access via [`Client::packages()`](crate::Client::packages).
pub struct PackagesApi<'a> {
    client: &'a Client,
}

impl<'a> PackagesApi<'a> {
    /// Create a new `PackagesApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// ListPackages lists all the packages owned by a given owner.
    pub async fn list_packages(
        &self,
        owner: &str,
        opt: ListPackagesOptions,
    ) -> crate::Result<(Vec<Package>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner])?;
        let path = format!("/packages/{}?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPackage gets the details of a specific package version.
    pub async fn get_package(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
        version: &str,
    ) -> crate::Result<(Package, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[
            owner,
            package_type,
            name,
            version,
        ])?;
        let path = format!(
            "/packages/{}/{}/{}/{}",
            escaped[0], escaped[1], escaped[2], escaped[3]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeletePackage deletes a specific package version.
    pub async fn delete_package(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
        version: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[
            owner,
            package_type,
            name,
            version,
        ])?;
        let path = format!(
            "/packages/{}/{}/{}/{}",
            escaped[0], escaped[1], escaped[2], escaped[3]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ListPackageFiles lists the files within a package.
    pub async fn list_package_files(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
        version: &str,
    ) -> crate::Result<(Vec<PackageFile>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[
            owner,
            package_type,
            name,
            version,
        ])?;
        let path = format!(
            "/packages/{}/{}/{}/{}/files",
            escaped[0], escaped[1], escaped[2], escaped[3]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetLatestPackage gets the latest version details of a package.
    pub async fn get_latest_package(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
    ) -> crate::Result<(Package, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, package_type, name])?;
        let path = format!(
            "/packages/{}/{}/{}/-/latest",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// LinkPackage links a package to a repository.
    pub async fn link_package(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
        repo_name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[
            owner,
            package_type,
            name,
            repo_name,
        ])?;
        let path = format!(
            "/packages/{}/{}/{}/-/link/{}",
            escaped[0], escaped[1], escaped[2], escaped[3]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// UnlinkPackage unlinks a package from a repository.
    pub async fn unlink_package(
        &self,
        owner: &str,
        package_type: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, package_type, name])?;
        let path = format!(
            "/packages/{}/{}/{}/-/unlink",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
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

    fn user_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "login": "owner",
            "login_name": "",
            "source_id": 0,
            "full_name": "",
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "last_login": null,
            "created": null,
            "restricted": false,
            "active": true,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0
        })
    }

    fn package_json() -> serde_json::Value {
        serde_json::json!({
            "id": 42,
            "owner": user_json(),
            "repository": null,
            "creator": user_json(),
            "type": "container",
            "name": "pkg",
            "version": "1.0.0",
            "created_at": "2026-03-18T00:00:00Z"
        })
    }

    #[tokio::test]
    async fn test_list_packages() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner"))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![package_json()]))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (packages, response) = client
            .packages()
            .list_packages("owner", Default::default())
            .await
            .unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].name, "pkg");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_list_packages_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .list_packages("owner", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_package() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(package_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (package, response) = client
            .packages()
            .get_package("owner", "container", "pkg", "1.0.0")
            .await
            .unwrap();
        assert_eq!(package.id, 42);
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_delete_package() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let response = client
            .packages()
            .delete_package("owner", "container", "pkg", "1.0.0")
            .await
            .unwrap();
        assert_eq!(response.status, 204);
    }

    #[tokio::test]
    async fn test_delete_package_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .delete_package("owner", "container", "pkg", "1.0.0")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_package_files() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0/files"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(vec![serde_json::json!({
                    "id": 7,
                    "size": 1024,
                    "name": "pkg.tar.gz",
                    "md5": "a",
                    "sha1": "b",
                    "sha256": "c",
                    "sha512": "d"
                })]),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (files, response) = client
            .packages()
            .list_package_files("owner", "container", "pkg", "1.0.0")
            .await
            .unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "pkg.tar.gz");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_list_package_files_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0/files"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .list_package_files("owner", "container", "pkg", "1.0.0")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_latest_package() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/-/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_json(package_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (package, response) = client
            .packages()
            .get_latest_package("owner", "container", "pkg")
            .await
            .unwrap();
        assert_eq!(package.version, "1.0.0");
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_get_latest_package_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/-/latest"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .get_latest_package("owner", "container", "pkg")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_link_package() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/packages/owner/container/pkg/-/link/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let response = client
            .packages()
            .link_package("owner", "container", "pkg", "repo")
            .await
            .unwrap();
        assert_eq!(response.status, 204);
    }

    #[tokio::test]
    async fn test_link_package_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/packages/owner/container/pkg/-/link/repo"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .link_package("owner", "container", "pkg", "repo")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_package() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/packages/owner/container/pkg/-/unlink"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let response = client
            .packages()
            .unlink_package("owner", "container", "pkg")
            .await
            .unwrap();
        assert_eq!(response.status, 204);
    }

    #[tokio::test]
    async fn test_unlink_package_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/packages/owner/container/pkg/-/unlink"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .unlink_package("owner", "container", "pkg")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_package_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/packages/owner/container/pkg/1%2E0%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .packages()
            .get_package("owner", "container", "pkg", "1.0.0")
            .await;
        assert!(result.is_err());
    }
}
