// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::future::Future;
use std::pin::Pin;

type CleanupTask = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[allow(dead_code)]
pub struct CleanupRegistry {
    tasks: Vec<CleanupTask>,
}

#[allow(dead_code)]
impl CleanupRegistry {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn register<F>(&mut self, task: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push(Box::pin(task));
    }

    pub async fn run_all(&mut self) {
        while let Some(task) = self.tasks.pop() {
            task.await;
        }
    }
}
