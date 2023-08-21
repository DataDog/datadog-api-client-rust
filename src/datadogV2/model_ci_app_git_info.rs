// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppGitInfo {
    /// The commit author email.
    #[serde(rename = "author_email", skip_serializing_if = "Option::is_none")]
    pub author_email: String,
    /// The commit author name.
    #[serde(rename = "author_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub author_name: Option<String>,
    /// The commit author timestamp in RFC3339 format.
    #[serde(rename = "author_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub author_time: Option<String>,
    /// The branch name (if a tag use the tag parameter).
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub branch: Option<String>,
    /// The commit timestamp in RFC3339 format.
    #[serde(rename = "commit_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub commit_time: Option<String>,
    /// The committer email.
    #[serde(rename = "committer_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub committer_email: Option<String>,
    /// The committer name.
    #[serde(rename = "committer_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub committer_name: Option<String>,
    /// The Git repository's default branch.
    #[serde(rename = "default_branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub default_branch: Option<String>,
    /// The commit message.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// The URL of the repository.
    #[serde(rename = "repository_url", skip_serializing_if = "Option::is_none")]
    pub repository_url: String,
    /// The git commit SHA.
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: String,
    /// The tag name (if a branch use the branch parameter).
    #[serde(rename = "tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tag: Option<String>,
}

