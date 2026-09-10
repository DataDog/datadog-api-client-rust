// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Per-file line coverage data including executable, covered, and added lines.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FileCoverageLines {
    /// Line numbers that were added in the specified scope (for example, in a PR diff).
    #[serde(rename = "added_lines")]
    pub added_lines: Option<Vec<i64>>,
    /// Line numbers that were covered by tests.
    #[serde(rename = "covered_lines")]
    pub covered_lines: Option<Vec<i64>>,
    /// Line numbers that are executable (can be covered).
    #[serde(rename = "executable_lines")]
    pub executable_lines: Option<Vec<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FileCoverageLines {
    pub fn new() -> FileCoverageLines {
        FileCoverageLines {
            added_lines: None,
            covered_lines: None,
            executable_lines: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn added_lines(mut self, value: Vec<i64>) -> Self {
        self.added_lines = Some(value);
        self
    }

    pub fn covered_lines(mut self, value: Vec<i64>) -> Self {
        self.covered_lines = Some(value);
        self
    }

    pub fn executable_lines(mut self, value: Vec<i64>) -> Self {
        self.executable_lines = Some(value);
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

impl Default for FileCoverageLines {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FileCoverageLines {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FileCoverageLinesVisitor;
        impl<'a> Visitor<'a> for FileCoverageLinesVisitor {
            type Value = FileCoverageLines;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut added_lines: Option<Vec<i64>> = None;
                let mut covered_lines: Option<Vec<i64>> = None;
                let mut executable_lines: Option<Vec<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "added_lines" => {
                            if v.is_null() {
                                continue;
                            }
                            added_lines =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "covered_lines" => {
                            if v.is_null() {
                                continue;
                            }
                            covered_lines =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "executable_lines" => {
                            if v.is_null() {
                                continue;
                            }
                            executable_lines =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FileCoverageLines {
                    added_lines,
                    covered_lines,
                    executable_lines,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FileCoverageLinesVisitor)
    }
}
