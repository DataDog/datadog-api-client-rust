// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a feature flag change suggestion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagSuggestionAttributes {
    /// The action proposed by the suggestion.
    #[serde(rename = "action")]
    pub action: String,
    /// The ID of the flag history entry on which the suggestion is based.
    #[serde(rename = "base_flag_history_id")]
    pub base_flag_history_id: uuid::Uuid,
    /// The comment supplied by the suggestion's creator.
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// The timestamp when the suggestion was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The ID of the user who created the suggestion.
    #[serde(rename = "created_by")]
    pub created_by: uuid::Uuid,
    /// The legacy review status. Use `lifecycle_status` to distinguish approval from application.
    #[serde(rename = "current_status")]
    pub current_status: crate::datadogV2::model::FeatureFlagSuggestionAttributesCurrentStatus,
    /// The value before the proposed change, serialized as a string.
    #[serde(rename = "current_value")]
    pub current_value: String,
    /// The timestamp when the suggestion was deleted.
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the user who deleted the suggestion.
    #[serde(rename = "deleted_by")]
    pub deleted_by: Option<uuid::Uuid>,
    /// The ID of the environment targeted by the change, if the change is environment-specific.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// The ID of the feature flag that the suggestion changes.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: uuid::Uuid,
    /// The lifecycle status of the suggestion.
    #[serde(rename = "lifecycle_status")]
    pub lifecycle_status: crate::datadogV2::model::FeatureFlagSuggestionAttributesLifecycleStatus,
    /// A message explaining why a suggestion was created.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The feature flag property to update.
    #[serde(rename = "property")]
    pub property: String,
    /// The proposed value, serialized as a string. Structured values, such as variants and allocations,
    /// are JSON-encoded strings.
    #[serde(rename = "suggestion")]
    pub suggestion: String,
    /// Metadata associated with a suggestion. Fields other than `variant_id` are set by the server.
    #[serde(rename = "suggestion_metadata")]
    pub suggestion_metadata: Option<crate::datadogV2::model::FeatureFlagSuggestionMetadata>,
    /// The timestamp when the suggestion was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the user who last updated the suggestion.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagSuggestionAttributes {
    pub fn new(
        action: String,
        base_flag_history_id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: uuid::Uuid,
        current_status: crate::datadogV2::model::FeatureFlagSuggestionAttributesCurrentStatus,
        current_value: String,
        feature_flag_id: uuid::Uuid,
        lifecycle_status: crate::datadogV2::model::FeatureFlagSuggestionAttributesLifecycleStatus,
        property: String,
        suggestion: String,
    ) -> FeatureFlagSuggestionAttributes {
        FeatureFlagSuggestionAttributes {
            action,
            base_flag_history_id,
            comment: None,
            created_at,
            created_by,
            current_status,
            current_value,
            deleted_at: None,
            deleted_by: None,
            environment_id: None,
            feature_flag_id,
            lifecycle_status,
            message: None,
            property,
            suggestion,
            suggestion_metadata: None,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn comment(mut self, value: String) -> Self {
        self.comment = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn deleted_by(mut self, value: uuid::Uuid) -> Self {
        self.deleted_by = Some(value);
        self
    }

    pub fn environment_id(mut self, value: uuid::Uuid) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn suggestion_metadata(
        mut self,
        value: crate::datadogV2::model::FeatureFlagSuggestionMetadata,
    ) -> Self {
        self.suggestion_metadata = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: uuid::Uuid) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for FeatureFlagSuggestionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagSuggestionAttributesVisitor;
        impl<'a> Visitor<'a> for FeatureFlagSuggestionAttributesVisitor {
            type Value = FeatureFlagSuggestionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut base_flag_history_id: Option<uuid::Uuid> = None;
                let mut comment: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut current_status: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionAttributesCurrentStatus,
                > = None;
                let mut current_value: Option<String> = None;
                let mut deleted_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_by: Option<uuid::Uuid> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut feature_flag_id: Option<uuid::Uuid> = None;
                let mut lifecycle_status: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionAttributesLifecycleStatus,
                > = None;
                let mut message: Option<String> = None;
                let mut property: Option<String> = None;
                let mut suggestion: Option<String> = None;
                let mut suggestion_metadata: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionMetadata,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "base_flag_history_id" => {
                            base_flag_history_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comment" => {
                            if v.is_null() {
                                continue;
                            }
                            comment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "current_status" => {
                            current_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _current_status) = current_status {
                                match _current_status {
                                    crate::datadogV2::model::FeatureFlagSuggestionAttributesCurrentStatus::UnparsedObject(_current_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "current_value" => {
                            current_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_by" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
                            feature_flag_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lifecycle_status" => {
                            lifecycle_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _lifecycle_status) = lifecycle_status {
                                match _lifecycle_status {
                                    crate::datadogV2::model::FeatureFlagSuggestionAttributesLifecycleStatus::UnparsedObject(_lifecycle_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suggestion" => {
                            suggestion = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suggestion_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            suggestion_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let base_flag_history_id = base_flag_history_id
                    .ok_or_else(|| M::Error::missing_field("base_flag_history_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let current_status =
                    current_status.ok_or_else(|| M::Error::missing_field("current_status"))?;
                let current_value =
                    current_value.ok_or_else(|| M::Error::missing_field("current_value"))?;
                let feature_flag_id =
                    feature_flag_id.ok_or_else(|| M::Error::missing_field("feature_flag_id"))?;
                let lifecycle_status =
                    lifecycle_status.ok_or_else(|| M::Error::missing_field("lifecycle_status"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;
                let suggestion = suggestion.ok_or_else(|| M::Error::missing_field("suggestion"))?;

                let content = FeatureFlagSuggestionAttributes {
                    action,
                    base_flag_history_id,
                    comment,
                    created_at,
                    created_by,
                    current_status,
                    current_value,
                    deleted_at,
                    deleted_by,
                    environment_id,
                    feature_flag_id,
                    lifecycle_status,
                    message,
                    property,
                    suggestion,
                    suggestion_metadata,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagSuggestionAttributesVisitor)
    }
}
