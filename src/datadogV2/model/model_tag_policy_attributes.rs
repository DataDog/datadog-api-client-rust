// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a tag policy resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagPolicyAttributes {
    /// The RFC 3339 timestamp at which the policy was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The identifier of the user who created the policy.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// The RFC 3339 timestamp at which the policy was soft-deleted. `null` if the policy has not been deleted. Only present when `include_deleted=true` is requested.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The identifier of the user who soft-deleted the policy. `null` if the policy has not been deleted.
    #[serde(
        rename = "deleted_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_by: Option<Option<String>>,
    /// Whether the policy is currently enforced.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The RFC 3339 timestamp at which the policy was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The identifier of the user who last modified the policy.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// When `true`, the policy matches tag values that do NOT match any of the supplied patterns.
    #[serde(rename = "negated")]
    pub negated: bool,
    /// Human-readable name for the tag policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// How the policy is enforced. `blocking` rejects telemetry that violates the policy.
    /// `surfacing` only highlights non-compliant telemetry without blocking it.
    #[serde(rename = "policy_type")]
    pub policy_type: crate::datadogV2::model::TagPolicyType,
    /// When `true`, telemetry without this tag is treated as a violation.
    #[serde(rename = "required")]
    pub required: bool,
    /// The scope the policy applies within.
    #[serde(rename = "scope")]
    pub scope: String,
    /// The telemetry source that a tag policy applies to.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::TagPolicySource,
    /// The tag key that the policy governs.
    #[serde(rename = "tag_key")]
    pub tag_key: String,
    /// The patterns that valid values for the tag key must match.
    #[serde(rename = "tag_value_patterns")]
    pub tag_value_patterns: Vec<String>,
    /// A monotonically increasing version counter that is incremented on each update.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagPolicyAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        enabled: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
        negated: bool,
        policy_name: String,
        policy_type: crate::datadogV2::model::TagPolicyType,
        required: bool,
        scope: String,
        source: crate::datadogV2::model::TagPolicySource,
        tag_key: String,
        tag_value_patterns: Vec<String>,
        version: i64,
    ) -> TagPolicyAttributes {
        TagPolicyAttributes {
            created_at,
            created_by,
            deleted_at: None,
            deleted_by: None,
            enabled,
            modified_at,
            modified_by,
            negated,
            policy_name,
            policy_type,
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

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn deleted_by(mut self, value: Option<String>) -> Self {
        self.deleted_by = Some(value);
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

impl<'de> Deserialize<'de> for TagPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for TagPolicyAttributesVisitor {
            type Value = TagPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut deleted_by: Option<Option<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut negated: Option<bool> = None;
                let mut policy_name: Option<String> = None;
                let mut policy_type: Option<crate::datadogV2::model::TagPolicyType> = None;
                let mut required: Option<bool> = None;
                let mut scope: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::TagPolicySource> = None;
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
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_by" => {
                            deleted_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "policy_type" => {
                            policy_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _policy_type) = policy_type {
                                match _policy_type {
                                    crate::datadogV2::model::TagPolicyType::UnparsedObject(
                                        _policy_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "required" => {
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
                let policy_type =
                    policy_type.ok_or_else(|| M::Error::missing_field("policy_type"))?;
                let required = required.ok_or_else(|| M::Error::missing_field("required"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let tag_key = tag_key.ok_or_else(|| M::Error::missing_field("tag_key"))?;
                let tag_value_patterns = tag_value_patterns
                    .ok_or_else(|| M::Error::missing_field("tag_value_patterns"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = TagPolicyAttributes {
                    created_at,
                    created_by,
                    deleted_at,
                    deleted_by,
                    enabled,
                    modified_at,
                    modified_by,
                    negated,
                    policy_name,
                    policy_type,
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

        deserializer.deserialize_any(TagPolicyAttributesVisitor)
    }
}
