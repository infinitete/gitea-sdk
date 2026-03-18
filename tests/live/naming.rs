// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn unique_name(prefix: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis();
    format!(
        "{prefix}-{pid}-{ms}",
        prefix = prefix,
        pid = process::id(),
        ms = millis
    )
}
