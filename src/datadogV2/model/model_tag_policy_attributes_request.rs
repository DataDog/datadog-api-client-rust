// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a tag policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyAttributesRequest {
    /// Whether the policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Whether the policy is negated.
    #[serde(rename = "negated")]
    pub negated: bool,
    /// The name of the tag policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// Whether the tag is required.
    #[serde(rename = "required")]
    pub required: bool,
    /// The scope of the tag policy.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The data source for the tag policy (e.g., logs, metrics).
    #[serde(rename = "source")]
    pub source: String,
    /// The tag key that the policy applies to.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// List of patterns that tag values must match.
    #[serde(rename = "tag_value_patterns")]
    pub tag_value_patterns: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagPolicyAttributesRequest {
    pub fn new(
        enabled: bool,
        negated: bool,
        policy_name: String,
        required: bool,
        scope: String,
        source: String,
        tag_key: String,
        tag_value_patterns: Vec<String>,
    ) -> TagPolicyAttributesRequest {
        TagPolicyAttributesRequest {
            enabled,
            negated,
            policy_name,
            required,
            scope,
            source,
            tag_key,
            tag_value_patterns,
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

impl<'de> Deserialize<'de> for TagPolicyAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyAttributesRequestVisitor;
        impl<'a> Visitor<'a> for TagPolicyAttributesRequestVisitor {
            type Value = TagPolicyAttributesRequest;

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
                let mut required: Option<bool> = None;
                let mut scope: Option<String> = None;
                let mut source: Option<String> = None;
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
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "negated" => {
                            negated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "policy_name" => {
                            policy_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let negated = negated.ok_or_else(|| M::Error::missing_field("negated"))?;
                let policy_name =
                    policy_name.ok_or_else(|| M::Error::missing_field("policy_name"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_value_patterns = tag_value_patterns
                    .ok_or_else(|| M::Error::missing_field("tag_value_patterns"))?;

                let content = TagPolicyAttributesRequest {
                    enabled,
                    negated,
                    policy_name,
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

        deserializer.deserialize_any(TagPolicyAttributesRequestVisitor)
    }
}
