// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a tag policy. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyAttributesUpdateRequest {
    /// Whether the policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Whether the policy is negated.
    #[serde(rename = "negated")]
    pub negated: Option<bool>,
    /// The name of the tag policy.
    #[serde(rename = "policy_name")]
    pub policy_name: Option<String>,
    /// Whether the tag is required.
    #[serde(rename = "required")]
    pub required: Option<bool>,
    /// The scope of the tag policy.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The data source for the tag policy (e.g., logs, metrics).
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// The tag key that the policy applies to.
    #[serde(rename = "tag_key")]
    pub tag_key: Option<String>,
    /// List of patterns that tag values must match.
    #[serde(rename = "tag_value_patterns")]
    pub tag_value_patterns: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagPolicyAttributesUpdateRequest {
    pub fn new() -> TagPolicyAttributesUpdateRequest {
        TagPolicyAttributesUpdateRequest {
            enabled: None,
            negated: None,
            policy_name: None,
            required: None,
            scope: None,
            source: None,
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

    pub fn negated(mut self, value: bool) -> Self {
        self.negated = Some(value);
        self
    }

    pub fn policy_name(mut self, value: String) -> Self {
        self.policy_name = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
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

impl Default for TagPolicyAttributesUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagPolicyAttributesUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyAttributesUpdateRequestVisitor;
        impl<'a> Visitor<'a> for TagPolicyAttributesUpdateRequestVisitor {
            type Value = TagPolicyAttributesUpdateRequest;

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
                            if v.is_null() {
                                continue;
                            }
                            policy_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required" => {
                            if v.is_null() {
                                continue;
                            }
                            required = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = TagPolicyAttributesUpdateRequest {
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

        deserializer.deserialize_any(TagPolicyAttributesUpdateRequestVisitor)
    }
}
