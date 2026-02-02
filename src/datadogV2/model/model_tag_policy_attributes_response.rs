// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a tag policy response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyAttributesResponse {
    /// Timestamp when the policy was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// User who created the policy.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Whether the policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Timestamp when the policy was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// User who last modified the policy.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
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
    /// The version of the tag policy.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagPolicyAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        enabled: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
        negated: bool,
        policy_name: String,
        required: bool,
        scope: String,
        source: String,
        tag_key: String,
        tag_value_patterns: Vec<String>,
        version: i64,
    ) -> TagPolicyAttributesResponse {
        TagPolicyAttributesResponse {
            created_at,
            created_by,
            enabled,
            modified_at,
            modified_by,
            negated,
            policy_name,
            required,
            scope,
            source,
            tag_key,
            tag_value_patterns,
            version,
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

impl<'de> Deserialize<'de> for TagPolicyAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyAttributesResponseVisitor;
        impl<'a> Visitor<'a> for TagPolicyAttributesResponseVisitor {
            type Value = TagPolicyAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut negated: Option<bool> = None;
                let mut policy_name: Option<String> = None;
                let mut required: Option<bool> = None;
                let mut scope: Option<String> = None;
                let mut source: Option<String> = None;
                let mut tag_key: Option<String> = None;
                let mut tag_value_patterns: Option<Vec<String>> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let negated = negated.ok_or_else(|| M::Error::missing_field("negated"))?;
                let policy_name =
                    policy_name.ok_or_else(|| M::Error::missing_field("policy_name"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_value_patterns = tag_value_patterns
                    .ok_or_else(|| M::Error::missing_field("tag_value_patterns"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = TagPolicyAttributesResponse {
                    created_at,
                    created_by,
                    enabled,
                    modified_at,
                    modified_by,
                    negated,
                    policy_name,
                    required,
                    scope,
                    source,
                    tag_key,
                    tag_value_patterns,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagPolicyAttributesResponseVisitor)
    }
}
