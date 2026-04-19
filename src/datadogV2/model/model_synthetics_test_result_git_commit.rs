// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the Git commit associated with the test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultGitCommit {
    /// A Git user (author or committer).
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::SyntheticsTestResultGitUser>,
    /// A Git user (author or committer).
    #[serde(rename = "committer")]
    pub committer: Option<crate::datadogV2::model::SyntheticsTestResultGitUser>,
    /// Commit message.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Commit SHA.
    #[serde(rename = "sha")]
    pub sha: Option<String>,
    /// URL of the commit.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultGitCommit {
    pub fn new() -> SyntheticsTestResultGitCommit {
        SyntheticsTestResultGitCommit {
            author: None,
            committer: None,
            message: None,
            sha: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV2::model::SyntheticsTestResultGitUser) -> Self {
        self.author = Some(value);
        self
    }

    pub fn committer(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultGitUser,
    ) -> Self {
        self.committer = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn sha(mut self, value: String) -> Self {
        self.sha = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestResultGitCommit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultGitCommit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultGitCommitVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultGitCommitVisitor {
            type Value = SyntheticsTestResultGitCommit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::SyntheticsTestResultGitUser> = None;
                let mut committer: Option<crate::datadogV2::model::SyntheticsTestResultGitUser> =
                    None;
                let mut message: Option<String> = None;
                let mut sha: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "committer" => {
                            if v.is_null() {
                                continue;
                            }
                            committer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sha" => {
                            if v.is_null() {
                                continue;
                            }
                            sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultGitCommit {
                    author,
                    committer,
                    message,
                    sha,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultGitCommitVisitor)
    }
}
