// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a rule based on a standard regex rule, whose pattern is maintained by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerStandardRegexRuleAttributes {
    /// Description of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Attributes excluded from the scan. If namespaces is provided, it has to be a sub-path of the namespaces array.
    #[serde(rename = "excluded_namespaces")]
    pub excluded_namespaces: Option<Vec<String>>,
    /// Object defining a set of keywords and a number of characters that help reduce noise.
    /// You can provide a list of keywords you would like to check within a defined proximity of the matching pattern.
    /// If any of the keywords are found within the proximity check, the match is kept.
    /// If none are found, the match is discarded.
    #[serde(rename = "included_keyword_configuration")]
    pub included_keyword_configuration:
        Option<crate::datadogV2::model::SensitiveDataScannerIncludedKeywordConfiguration>,
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
    /// Severity of the issues found by the rule.
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::SensitiveDataScannerRulePriority>,
    /// Type of a rule based on a standard regex rule.
    #[serde(rename = "rule_type")]
    pub rule_type: crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleType,
    /// Object describing suppressions for a rule, or the recommended suppression lists published by a standard pattern.
    /// There are three types of suppressions, `starts_with`, `ends_with`, and `exact_match`.
    /// Rules based on the Datadog library can use recommended or custom suppressions. Custom rules can only use custom suppressions.
    /// Suppressed matches are not obfuscated, counted in metrics, or displayed in the Findings page.
    #[serde(rename = "suppressions")]
    pub suppressions: Option<crate::datadogV2::model::SensitiveDataScannerSuppressions>,
    /// List of tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Object describing how the scanned event will be replaced.
    #[serde(rename = "text_replacement")]
    pub text_replacement: Option<crate::datadogV2::model::SensitiveDataScannerTextReplacement>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerStandardRegexRuleAttributes {
    pub fn new(
        rule_type: crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleType,
    ) -> SensitiveDataScannerStandardRegexRuleAttributes {
        SensitiveDataScannerStandardRegexRuleAttributes {
            description: None,
            excluded_namespaces: None,
            included_keyword_configuration: None,
            is_enabled: None,
            name: None,
            namespaces: None,
            priority: None,
            rule_type,
            suppressions: None,
            tags: None,
            text_replacement: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn included_keyword_configuration(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerIncludedKeywordConfiguration,
    ) -> Self {
        self.included_keyword_configuration = Some(value);
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

    pub fn priority(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerRulePriority,
    ) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn suppressions(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerSuppressions,
    ) -> Self {
        self.suppressions = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerStandardRegexRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerStandardRegexRuleAttributesVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerStandardRegexRuleAttributesVisitor {
            type Value = SensitiveDataScannerStandardRegexRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut excluded_namespaces: Option<Vec<String>> = None;
                let mut included_keyword_configuration: Option<
                    crate::datadogV2::model::SensitiveDataScannerIncludedKeywordConfiguration,
                > = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut namespaces: Option<Vec<String>> = None;
                let mut priority: Option<
                    crate::datadogV2::model::SensitiveDataScannerRulePriority,
                > = None;
                let mut rule_type: Option<
                    crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleType,
                > = None;
                let mut suppressions: Option<
                    crate::datadogV2::model::SensitiveDataScannerSuppressions,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut text_replacement: Option<
                    crate::datadogV2::model::SensitiveDataScannerTextReplacement,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "included_keyword_configuration" => {
                            if v.is_null() {
                                continue;
                            }
                            included_keyword_configuration =
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
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::SensitiveDataScannerRulePriority::UnparsedObject(_priority) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rule_type" => {
                            rule_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule_type) = rule_type {
                                match _rule_type {
                                    crate::datadogV2::model::SensitiveDataScannerStandardRegexRuleType::UnparsedObject(_rule_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "suppressions" => {
                            if v.is_null() {
                                continue;
                            }
                            suppressions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rule_type = rule_type.ok_or_else(|| M::Error::missing_field("rule_type"))?;

                let content = SensitiveDataScannerStandardRegexRuleAttributes {
                    description,
                    excluded_namespaces,
                    included_keyword_configuration,
                    is_enabled,
                    name,
                    namespaces,
                    priority,
                    rule_type,
                    suppressions,
                    tags,
                    text_replacement,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerStandardRegexRuleAttributesVisitor)
    }
}
