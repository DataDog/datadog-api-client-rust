// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
/// Note that either `tag` or `branch` has to be provided, but not both.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn author_name(mut self, value: Option<String>) -> Self {
        self.author_name = Some(value);
        self
    }

    pub fn author_time(mut self, value: Option<String>) -> Self {
        self.author_time = Some(value);
        self
    }

    pub fn branch(mut self, value: Option<String>) -> Self {
        self.branch = Some(value);
        self
    }

    pub fn commit_time(mut self, value: Option<String>) -> Self {
        self.commit_time = Some(value);
        self
    }

    pub fn committer_email(mut self, value: Option<String>) -> Self {
        self.committer_email = Some(value);
        self
    }

    pub fn committer_name(mut self, value: Option<String>) -> Self {
        self.committer_name = Some(value);
        self
    }

    pub fn default_branch(mut self, value: Option<String>) -> Self {
        self.default_branch = Some(value);
        self
    }

    pub fn message(mut self, value: Option<String>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn tag(mut self, value: Option<String>) -> Self {
        self.tag = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CIAppGitInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppGitInfoVisitor;
        impl<'a> Visitor<'a> for CIAppGitInfoVisitor {
            type Value = CIAppGitInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author_email: Option<String> = None;
                let mut author_name: Option<Option<String>> = None;
                let mut author_time: Option<Option<String>> = None;
                let mut branch: Option<Option<String>> = None;
                let mut commit_time: Option<Option<String>> = None;
                let mut committer_email: Option<Option<String>> = None;
                let mut committer_name: Option<Option<String>> = None;
                let mut default_branch: Option<Option<String>> = None;
                let mut message: Option<Option<String>> = None;
                let mut repository_url: Option<String> = None;
                let mut sha: Option<String> = None;
                let mut tag: Option<Option<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author_email" => {
                            author_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "author_name" => {
                            author_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "author_time" => {
                            author_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "branch" => {
                            branch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commit_time" => {
                            commit_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committer_email" => {
                            committer_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committer_name" => {
                            committer_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_branch" => {
                            default_branch =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_url" => {
                            repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sha" => {
                            sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag" => {
                            tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let author_email =
                    author_email.ok_or_else(|| M::Error::missing_field("author_email"))?;
                let repository_url =
                    repository_url.ok_or_else(|| M::Error::missing_field("repository_url"))?;
                let sha = sha.ok_or_else(|| M::Error::missing_field("sha"))?;

                let content = CIAppGitInfo {
                    author_email,
                    author_name,
                    author_time,
                    branch,
                    commit_time,
                    committer_email,
                    committer_name,
                    default_branch,
                    message,
                    repository_url,
                    sha,
                    tag,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppGitInfoVisitor)
    }
}
