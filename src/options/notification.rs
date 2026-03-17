// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::ListOptions;
use crate::types::enums::{NotifyStatus, NotifySubjectType};
use time::OffsetDateTime;

#[derive(Debug, Clone, Default)]
pub struct ListNotificationOptions {
    pub list_options: ListOptions,
    pub since: Option<OffsetDateTime>,
    pub before: Option<OffsetDateTime>,
    pub status: Vec<NotifyStatus>,
    pub subject_types: Vec<NotifySubjectType>,
}

impl crate::pagination::QueryEncode for ListNotificationOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(since) = self.since {
            out.push_str(&format!(
                "&since={}",
                since
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap()
            ));
        }
        if let Some(before) = self.before {
            out.push_str(&format!(
                "&before={}",
                before
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap()
            ));
        }
        for s in &self.status {
            out.push_str(&format!("&status-types={}", s.as_ref()));
        }
        for s in &self.subject_types {
            out.push_str(&format!("&subject-type={}", s.as_ref()));
        }
        out
    }
}

#[derive(Debug, Clone, Default)]
pub struct MarkNotificationOptions {
    pub last_read_at: Option<OffsetDateTime>,
    pub status: Vec<NotifyStatus>,
    pub to_status: Option<NotifyStatus>,
}

impl crate::pagination::QueryEncode for MarkNotificationOptions {
    fn query_encode(&self) -> String {
        let mut out = String::new();
        if let Some(last_read) = self.last_read_at {
            out.push_str(&format!(
                "last_read_at={}",
                last_read
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap()
            ));
        }
        for s in &self.status {
            if !out.is_empty() {
                out.push('&');
            }
            out.push_str(&format!("status-types={}", s.as_ref()));
        }
        if let Some(to_status) = self.to_status {
            if !out.is_empty() {
                out.push('&');
            }
            out.push_str(&format!("to-status={}", to_status.as_ref()));
        }
        out
    }
}
