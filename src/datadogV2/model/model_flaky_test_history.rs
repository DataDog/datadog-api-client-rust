// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single history entry representing a status change for a flaky test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestHistory {
    /// The commit SHA associated with this status change. Will be an empty string if the commit SHA is not available.
    #[serde(rename = "commit_sha")]
    pub commit_sha: String,
    /// The test status at this point in history.
    #[serde(rename = "status")]
    pub status: String,
    /// Unix timestamp in milliseconds when this status change occurred.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestHistory {
    pub fn new(commit_sha: String, status: String, timestamp: i64) -> FlakyTestHistory {
        FlakyTestHistory {
            commit_sha,
            status,
            timestamp,
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

impl<'de> Deserialize<'de> for FlakyTestHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestHistoryVisitor;
        impl<'a> Visitor<'a> for FlakyTestHistoryVisitor {
            type Value = FlakyTestHistory;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commit_sha: Option<String> = None;
                let mut status: Option<String> = None;
                let mut timestamp: Option<i64> = None;
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
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let commit_sha = commit_sha.ok_or_else(|| M::Error::missing_field("commit_sha"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;

                let content = FlakyTestHistory {
                    commit_sha,
                    status,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestHistoryVisitor)
    }
}
