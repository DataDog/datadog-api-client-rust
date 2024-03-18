// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Sensitive Data Scanner standard pattern.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerStandardPatternAttributes {
    /// Description of the standard pattern.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// List of included keywords.
    #[serde(rename = "included_keywords")]
    pub included_keywords: Option<Vec<String>>,
    /// Name of the standard pattern.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Regex to match.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Integer from 1 (high) to 5 (low) indicating standard pattern issue severity.
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// List of tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerStandardPatternAttributes {
    pub fn new() -> SensitiveDataScannerStandardPatternAttributes {
        SensitiveDataScannerStandardPatternAttributes {
            description: None,
            included_keywords: None,
            name: None,
            pattern: None,
            priority: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn included_keywords(mut self, value: Vec<String>) -> Self {
        self.included_keywords = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerStandardPatternAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerStandardPatternAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerStandardPatternAttributesVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerStandardPatternAttributesVisitor {
            type Value = SensitiveDataScannerStandardPatternAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut included_keywords: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut pattern: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included_keywords" => {
                            if v.is_null() {
                                continue;
                            }
                            included_keywords =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerStandardPatternAttributes {
                    description,
                    included_keywords,
                    name,
                    pattern,
                    priority,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerStandardPatternAttributesVisitor)
    }
}
