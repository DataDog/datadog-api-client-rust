// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes that can be supplied when creating a tag policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyCreateAttributes {
    /// Whether the policy is currently enforced. Defaults to `true` for newly created policies.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// When `true`, the policy matches tag values that do NOT match any of the supplied patterns. Defaults to `false`.
    #[serde(rename = "negated")]
    pub negated: Option<bool>,
    /// Human-readable name for the tag policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// The policy type allowed when creating a tag policy. Only `surfacing` is accepted at
    /// creation time.
    #[serde(rename = "policy_type")]
    pub policy_type: crate::datadogV2::model::TagPolicyCreateType,
    /// When `true`, telemetry without this tag is treated as a violation. Defaults to `false`.
    #[serde(rename = "required")]
    pub required: Option<bool>,
    /// The scope the policy applies within. Typically an environment, team, or
    /// organization-level identifier used to limit where the policy is enforced.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The telemetry source that a tag policy applies to.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::TagPolicySource,
    /// The tag key that the policy governs (for example, `service`).
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

impl TagPolicyCreateAttributes {
    pub fn new(
        policy_name: String,
        policy_type: crate::datadogV2::model::TagPolicyCreateType,
        scope: String,
        source: crate::datadogV2::model::TagPolicySource,
        tag_key: String,
        tag_value_patterns: Vec<String>,
    ) -> TagPolicyCreateAttributes {
        TagPolicyCreateAttributes {
            enabled: None,
            negated: None,
            policy_name,
            policy_type,
            required: None,
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

impl<'de> Deserialize<'de> for TagPolicyCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyCreateAttributesVisitor;
        impl<'a> Visitor<'a> for TagPolicyCreateAttributesVisitor {
            type Value = TagPolicyCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut negated: Option<bool> = None;
                let mut policy_name: Option<String> = None;
                let mut policy_type: Option<crate::datadogV2::model::TagPolicyCreateType> = None;
                let mut required: Option<bool> = None;
                let mut scope: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::TagPolicySource> = None;
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
                        "negated" => {
                            if v.is_null() {
                                continue;
                            }
                            negated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_name" => {
                            policy_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_type" => {
                            policy_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _policy_type) = policy_type {
                                match _policy_type {
                                    crate::datadogV2::model::TagPolicyCreateType::UnparsedObject(_policy_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "required" => {
                            if v.is_null() {
                                continue;
                            }
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::TagPolicySource::UnparsedObject(
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
                let policy_name =
                    policy_name.ok_or_else(|| M::Error::missing_field("policy_name"))?;
                let policy_type =
                    policy_type.ok_or_else(|| M::Error::missing_field("policy_type"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_value_patterns = tag_value_patterns
                    .ok_or_else(|| M::Error::missing_field("tag_value_patterns"))?;

                let content = TagPolicyCreateAttributes {
                    enabled,
                    negated,
                    policy_name,
                    policy_type,
                    required,
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

        deserializer.deserialize_any(TagPolicyCreateAttributesVisitor)
    }
}
