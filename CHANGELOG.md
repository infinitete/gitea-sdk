# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-18

### Added
- Async Gitea API client with builder pattern
- 15 API modules: repos, issues, pulls, orgs, users, admin, hooks, notifications, actions, releases, settings, oauth2, packages, miscellaneous, activitypub, status
- 6 authentication mechanisms: token, basic auth, OTP, sudo, SSH cert signing, SSH pubkey signing
- Pagination support with `ListOptions` and `QueryEncode` trait
- Server version detection and constraint checking
- Webhook signature verification (HMAC-SHA256)
- `time::OffsetDateTime` for all timestamp fields
- Wiremock-based unit tests and live integration test suite
