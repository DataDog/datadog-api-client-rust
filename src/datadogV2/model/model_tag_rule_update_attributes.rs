// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mutable attributes of a tag rule. Each field is optional; omitting a field leaves its
/// current value unchanged. The `source` of a rule cannot be changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagRuleUpdateAttributes {
    /// Whether the rule is currently enforced.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Human-readable name for the tag rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// When `true`, the rule matches tag values that do NOT match any of the supplied patterns.
    #[serde(rename = "negated")]
    pub negated: Option<bool>,
    /// When `true`, telemetry without this tag is treated as a violation.
    #[serde(rename = "required")]
    pub required: Option<bool>,
    /// How the rule is enforced. `blocking` rejects telemetry that violates the rule.
    /// `surfacing` only highlights non-compliant telemetry without blocking it.
    #[serde(rename = "rule_type")]
    pub rule_type: Option<crate::datadogV2::model::TagRuleType>,
    /// The scope the rule applies within.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The tag key that the rule governs.
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    /// One or more patterns that valid values for the tag key must match.
    #[serde(rename = "tag_value_patterns")]
    pub tag_value_patterns: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagRuleUpdateAttributes {
    pub fn new() -> TagRuleUpdateAttributes {
        TagRuleUpdateAttributes {
            enabled: None,
            name: None,
            negated: None,
            required: None,
            rule_type: None,
            scope: None,
            tag_key: None,
            tag_value_patterns: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn negated(mut self, value: bool) -> Self {
        self.negated = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn rule_type(mut self, value: crate::datadogV2::model::TagRuleType) -> Self {
        self.rule_type = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn tag_key(mut self, value: String) -> Self {
        self.tag_key = Some(value);
        self
    }

    pub fn tag_value_patterns(mut self, value: Vec<String>) -> Self {
        self.tag_value_patterns = Some(value);
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

impl Default for TagRuleUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagRuleUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagRuleUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for TagRuleUpdateAttributesVisitor {
            type Value = TagRuleUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut negated: Option<bool> = None;
                let mut required: Option<bool> = None;
                let mut rule_type: Option<crate::datadogV2::model::TagRuleType> = None;
                let mut scope: Option<String> = None;
                let mut tag_key: Option<String> = None;
                let mut tag_value_patterns: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "negated" => {
                            if v.is_null() {
                                continue;
                            }
                            negated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            if v.is_null() {
                                continue;
                            }
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_type" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule_type) = rule_type {
                                match _rule_type {
                                    crate::datadogV2::model::TagRuleType::UnparsedObject(
                                        _rule_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_key" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_value_patterns" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_value_patterns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TagRuleUpdateAttributes {
                    enabled,
                    name,
                    negated,
                    required,
                    rule_type,
                    scope,
                    tag_key,
                    tag_value_patterns,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagRuleUpdateAttributesVisitor)
    }
}
