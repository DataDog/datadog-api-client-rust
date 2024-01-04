// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Git info for DORA Metrics events.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORAGitInfo {
    /// Git Commit SHA.
    #[serde(rename = "commit_sha")]
    pub commit_sha: String,
    /// Git Repository URL
    #[serde(rename = "repository_url")]
    pub repository_url: String,
}

impl DORAGitInfo {
    pub fn new(commit_sha: String, repository_url: String) -> DORAGitInfo {
        DORAGitInfo {
            commit_sha,
            repository_url,
        }
    }
}
