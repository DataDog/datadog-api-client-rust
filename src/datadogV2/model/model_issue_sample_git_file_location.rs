// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Location of a stack frame in a repository file. Empty when the source code is unavailable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueSampleGitFileLocation {
    /// Column of the location, starting at 1.
    #[serde(rename = "column")]
    pub column: Option<i64>,
    /// Path of the file in the repository.
    #[serde(rename = "filepath")]
    pub filepath: Option<String>,
    /// First line of the location, starting at 1.
    #[serde(rename = "first_line")]
    pub first_line: Option<i64>,
    /// Last line of the location, starting at 1.
    #[serde(rename = "last_line")]
    pub last_line: Option<i64>,
    /// Git reference of the file, usually a commit SHA.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Repository identifier.
    #[serde(rename = "repo_id")]
    pub repo_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueSampleGitFileLocation {
    pub fn new() -> IssueSampleGitFileLocation {
        IssueSampleGitFileLocation {
            column: None,
            filepath: None,
            first_line: None,
            last_line: None,
            ref_: None,
            repo_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn filepath(mut self, value: String) -> Self {
        self.filepath = Some(value);
        self
    }

    pub fn first_line(mut self, value: i64) -> Self {
        self.first_line = Some(value);
        self
    }

    pub fn last_line(mut self, value: i64) -> Self {
        self.last_line = Some(value);
        self
    }

    pub fn ref_(mut self, value: String) -> Self {
        self.ref_ = Some(value);
        self
    }

    pub fn repo_id(mut self, value: String) -> Self {
        self.repo_id = Some(value);
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

impl Default for IssueSampleGitFileLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueSampleGitFileLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueSampleGitFileLocationVisitor;
        impl<'a> Visitor<'a> for IssueSampleGitFileLocationVisitor {
            type Value = IssueSampleGitFileLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<i64> = None;
                let mut filepath: Option<String> = None;
                let mut first_line: Option<i64> = None;
                let mut last_line: Option<i64> = None;
                let mut ref_: Option<String> = None;
                let mut repo_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filepath" => {
                            if v.is_null() {
                                continue;
                            }
                            filepath = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_line" => {
                            if v.is_null() {
                                continue;
                            }
                            first_line = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_line" => {
                            if v.is_null() {
                                continue;
                            }
                            last_line = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ref" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repo_id" => {
                            if v.is_null() {
                                continue;
                            }
                            repo_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssueSampleGitFileLocation {
                    column,
                    filepath,
                    first_line,
                    last_line,
                    ref_,
                    repo_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueSampleGitFileLocationVisitor)
    }
}
