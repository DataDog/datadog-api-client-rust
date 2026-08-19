// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that can be supplied when creating a tag rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagRuleCreateAttributes {
    /// Whether the rule is currently enforced. Defaults to `true` for newly created rules.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Human-readable name for the tag rule.
    #[serde(rename = "name")]
    pub name: String,
    /// When `true`, the rule matches tag values that do NOT match any of the supplied patterns. Defaults to `false`.
    #[serde(rename = "negated")]
    pub negated: Option<bool>,
    /// When `true`, telemetry without this tag is treated as a violation. Defaults to `false`.
    #[serde(rename = "required")]
    pub required: Option<bool>,
    /// The rule type allowed when creating a tag rule. Only `surfacing` is accepted at
    /// creation time.
    #[serde(rename = "rule_type")]
    pub rule_type: crate::datadogV2::model::TagRuleCreateType,
    /// The scope the rule applies within. Typically an environment, team, or
    /// organization-level identifier used to limit where the rule is enforced.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The telemetry source that a tag rule applies to.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::TagRuleSource,
    /// The tag key that the rule governs (for example, `service`).
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// One or more patterns that valid values for the tag key must match. At least one
    /// pattern is required.
    #[serde(rename = "tag_value_patterns")]
    pub tag_value_patterns: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagRuleCreateAttributes {
    pub fn new(
        name: String,
        rule_type: crate::datadogV2::model::TagRuleCreateType,
        scope: String,
        source: crate::datadogV2::model::TagRuleSource,
        tag_key: String,
        tag_value_patterns: Vec<String>,
    ) -> TagRuleCreateAttributes {
        TagRuleCreateAttributes {
            enabled: None,
            name,
            negated: None,
            required: None,
            rule_type,
            scope,
            source,
            tag_key,
            tag_value_patterns,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TagRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TagRuleCreateAttributesVisitor {
            type Value = TagRuleCreateAttributes;

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
                let mut rule_type: Option<crate::datadogV2::model::TagRuleCreateType> = None;
                let mut scope: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::TagRuleSource> = None;
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
                            rule_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule_type) = rule_type {
                                match _rule_type {
                                    crate::datadogV2::model::TagRuleCreateType::UnparsedObject(
                                        _rule_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::TagRuleSource::UnparsedObject(
                                        _source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tag_key" => {
                            tag_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_value_patterns" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rule_type = rule_type.ok_or_else(|| M::Error::missing_field("rule_type"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_value_patterns = tag_value_patterns
                    .ok_or_else(|| M::Error::missing_field("tag_value_patterns"))?;

                let content = TagRuleCreateAttributes {
                    enabled,
                    name,
                    negated,
                    required,
                    rule_type,
                    scope,
                    source,
                    tag_key,
                    tag_value_patterns,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagRuleCreateAttributesVisitor)
    }
}
