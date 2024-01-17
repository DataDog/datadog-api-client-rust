// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
/// Note that either `tag` or `branch` has to be provided, but not both.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppGitInfo {
    /// The commit author email.
    #[serde(rename = "author_email")]
    pub author_email: String,
    /// The commit author name.
    #[serde(
        rename = "author_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub author_name: Option<Option<String>>,
    /// The commit author timestamp in RFC3339 format.
    #[serde(
        rename = "author_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub author_time: Option<Option<String>>,
    /// The branch name (if a tag use the tag parameter).
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option")]
    pub branch: Option<Option<String>>,
    /// The commit timestamp in RFC3339 format.
    #[serde(
        rename = "commit_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub commit_time: Option<Option<String>>,
    /// The committer email.
    #[serde(
        rename = "committer_email",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub committer_email: Option<Option<String>>,
    /// The committer name.
    #[serde(
        rename = "committer_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub committer_name: Option<Option<String>>,
    /// The Git repository's default branch.
    #[serde(
        rename = "default_branch",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_branch: Option<Option<String>>,
    /// The commit message.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub message: Option<Option<String>>,
    /// The URL of the repository.
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    /// The git commit SHA.
    #[serde(rename = "sha")]
    pub sha: String,
    /// The tag name (if a branch use the branch parameter).
    #[serde(rename = "tag", default, with = "::serde_with::rust::double_option")]
    pub tag: Option<Option<String>>,
}

impl CIAppGitInfo {
    pub fn new(author_email: String, repository_url: String, sha: String) -> CIAppGitInfo {
        CIAppGitInfo {
            author_email,
            author_name: None,
            author_time: None,
            branch: None,
            commit_time: None,
            committer_email: None,
            committer_name: None,
            default_branch: None,
            message: None,
            repository_url,
            sha,
            tag: None,
        }
    }
}
