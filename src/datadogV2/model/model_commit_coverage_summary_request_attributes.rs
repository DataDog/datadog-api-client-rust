// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for requesting code coverage summary for a commit.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitCoverageSummaryRequestAttributes {
    /// The commit SHA (40-character hexadecimal string).
    #[serde(rename = "commit_sha")]
    pub commit_sha: String,
    /// The repository identifier.
    #[serde(rename = "repository_id")]
    pub repository_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitCoverageSummaryRequestAttributes {
    pub fn new(
        commit_sha: String,
        repository_id: String,
    ) -> CommitCoverageSummaryRequestAttributes {
        CommitCoverageSummaryRequestAttributes {
            commit_sha,
            repository_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CommitCoverageSummaryRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitCoverageSummaryRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CommitCoverageSummaryRequestAttributesVisitor {
            type Value = CommitCoverageSummaryRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commit_sha: Option<String> = None;
                let mut repository_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commit_sha" => {
                            commit_sha = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository_id" => {
                            repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let commit_sha = commit_sha.ok_or_else(|| M::Error::missing_field("commit_sha"))?;
                let repository_id =
                    repository_id.ok_or_else(|| M::Error::missing_field("repository_id"))?;

                let content = CommitCoverageSummaryRequestAttributes {
                    commit_sha,
                    repository_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitCoverageSummaryRequestAttributesVisitor)
    }
}
