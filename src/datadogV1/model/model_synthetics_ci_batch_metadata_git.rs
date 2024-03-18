// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Git information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsCIBatchMetadataGit {
    /// Branch name.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// The commit SHA.
    #[serde(rename = "commitSha")]
    pub commit_sha: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsCIBatchMetadataGit {
    pub fn new() -> SyntheticsCIBatchMetadataGit {
        SyntheticsCIBatchMetadataGit {
            branch: None,
            commit_sha: None,
            _unparsed: false,
        }
    }

    pub fn branch(mut self, value: String) -> Self {
        self.branch = Some(value);
        self
    }

    pub fn commit_sha(mut self, value: String) -> Self {
        self.commit_sha = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadataGit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsCIBatchMetadataGit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCIBatchMetadataGitVisitor;
        impl<'a> Visitor<'a> for SyntheticsCIBatchMetadataGitVisitor {
            type Value = SyntheticsCIBatchMetadataGit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branch: Option<String> = None;
                let mut commit_sha: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branch" => {
                            if v.is_null() {
                                continue;
                            }
                            branch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commitSha" => {
                            if v.is_null() {
                                continue;
                            }
                            commit_sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsCIBatchMetadataGit {
                    branch,
                    commit_sha,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCIBatchMetadataGitVisitor)
    }
}
