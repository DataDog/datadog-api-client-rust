// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the per-file code coverage response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilesCoverageAttributes {
    /// The SHA of the base commit used for comparison (for example, the merge base for a PR).
    #[serde(rename = "base_commit_sha")]
    pub base_commit_sha: Option<String>,
    /// Unix timestamp (milliseconds) of the coverage event.
    #[serde(rename = "event_timestamp")]
    pub event_timestamp: Option<i64>,
    /// Map of file paths to per-file coverage line data.
    #[serde(rename = "files")]
    pub files:
        Option<std::collections::BTreeMap<String, crate::datadogV2::model::FileCoverageLines>>,
    /// The SHA of the head commit for which coverage was evaluated.
    #[serde(rename = "head_commit_sha")]
    pub head_commit_sha: Option<String>,
    /// Number of coverage reports evaluated.
    #[serde(rename = "report_count")]
    pub report_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FilesCoverageAttributes {
    pub fn new() -> FilesCoverageAttributes {
        FilesCoverageAttributes {
            base_commit_sha: None,
            event_timestamp: None,
            files: None,
            head_commit_sha: None,
            report_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn base_commit_sha(mut self, value: String) -> Self {
        self.base_commit_sha = Some(value);
        self
    }

    pub fn event_timestamp(mut self, value: i64) -> Self {
        self.event_timestamp = Some(value);
        self
    }

    pub fn files(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::FileCoverageLines>,
    ) -> Self {
        self.files = Some(value);
        self
    }

    pub fn head_commit_sha(mut self, value: String) -> Self {
        self.head_commit_sha = Some(value);
        self
    }

    pub fn report_count(mut self, value: i64) -> Self {
        self.report_count = Some(value);
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

impl Default for FilesCoverageAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FilesCoverageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FilesCoverageAttributesVisitor;
        impl<'a> Visitor<'a> for FilesCoverageAttributesVisitor {
            type Value = FilesCoverageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_commit_sha: Option<String> = None;
                let mut event_timestamp: Option<i64> = None;
                let mut files: Option<
                    std::collections::BTreeMap<String, crate::datadogV2::model::FileCoverageLines>,
                > = None;
                let mut head_commit_sha: Option<String> = None;
                let mut report_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_commit_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            base_commit_sha =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            event_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "files" => {
                            if v.is_null() {
                                continue;
                            }
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "head_commit_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            head_commit_sha =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_count" => {
                            if v.is_null() {
                                continue;
                            }
                            report_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FilesCoverageAttributes {
                    base_commit_sha,
                    event_timestamp,
                    files,
                    head_commit_sha,
                    report_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FilesCoverageAttributesVisitor)
    }
}
