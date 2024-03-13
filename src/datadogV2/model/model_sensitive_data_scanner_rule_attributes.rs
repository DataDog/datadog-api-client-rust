// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the Sensitive Data Scanner rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerRuleAttributes {
    /// Description of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Attributes excluded from the scan. If namespaces is provided, it has to be a sub-path of the namespaces array.
    #[serde(rename = "excluded_namespaces")]
    pub excluded_namespaces: Option<Vec<String>>,
    /// Whether or not the rule is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Attributes included in the scan. If namespaces is empty or missing, all attributes except excluded_namespaces are scanned.
    /// If both are missing the whole event is scanned.
    #[serde(rename = "namespaces")]
    pub namespaces: Option<Vec<String>>,
    /// Not included if there is a relationship to a standard pattern.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Integer from 1 (high) to 5 (low) indicating rule issue severity.
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// List of tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Object describing how the scanned event will be replaced.
    #[serde(rename = "text_replacement")]
    pub text_replacement: Option<crate::datadogV2::model::SensitiveDataScannerTextReplacement>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerRuleAttributes {
    pub fn new() -> SensitiveDataScannerRuleAttributes {
        SensitiveDataScannerRuleAttributes {
            description: None,
            excluded_namespaces: None,
            is_enabled: None,
            name: None,
            namespaces: None,
            pattern: None,
            priority: None,
            tags: None,
            text_replacement: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn excluded_namespaces(mut self, value: Vec<String>) -> Self {
        self.excluded_namespaces = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn namespaces(mut self, value: Vec<String>) -> Self {
        self.namespaces = Some(value);
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

    pub fn text_replacement(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerTextReplacement,
    ) -> Self {
        self.text_replacement = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerRuleAttributesVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerRuleAttributesVisitor {
            type Value = SensitiveDataScannerRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut excluded_namespaces: Option<Vec<String>> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut namespaces: Option<Vec<String>> = None;
                let mut pattern: Option<String> = None;
                let mut priority: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut text_replacement: Option<
                    crate::datadogV2::model::SensitiveDataScannerTextReplacement,
                > = None;
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
                        "excluded_namespaces" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_namespaces =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespaces" => {
                            if v.is_null() {
                                continue;
                            }
                            namespaces = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "text_replacement" => {
                            if v.is_null() {
                                continue;
                            }
                            text_replacement =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerRuleAttributes {
                    description,
                    excluded_namespaces,
                    is_enabled,
                    name,
                    namespaces,
                    pattern,
                    priority,
                    tags,
                    text_replacement,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerRuleAttributesVisitor)
    }
}
