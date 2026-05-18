// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a flag suggestion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlagSuggestionAttributes {
    /// The type of change action for a suggestion.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::FlagSuggestionAction,
    /// The flag history version this suggestion was based on.
    #[serde(rename = "base_flag_history_id")]
    pub base_flag_history_id: Option<uuid::Uuid>,
    /// Optional comment from the requester.
    #[serde(
        rename = "comment",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub comment: Option<Option<String>>,
    /// When the suggestion was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// UUID of the user who created the suggestion.
    #[serde(rename = "created_by")]
    pub created_by: uuid::Uuid,
    /// The status of a flag suggestion.
    #[serde(rename = "current_status")]
    pub current_status: crate::datadogV2::model::FlagSuggestionStatus,
    /// The current value before the suggested change (empty string for flag-level actions like archive).
    #[serde(rename = "current_value")]
    pub current_value: Option<String>,
    /// When the suggestion was soft-deleted.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// UUID of the user who deleted the suggestion.
    #[serde(
        rename = "deleted_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_by: Option<Option<String>>,
    /// The environment ID for environment-scoped suggestions. Null for flag-level changes.
    #[serde(
        rename = "environment_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub environment_id: Option<Option<String>>,
    /// The ID of the feature flag this suggestion applies to.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: uuid::Uuid,
    /// Human-readable message about the suggestion (populated on auto-created suggestions).
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The flag property being changed.
    #[serde(rename = "property")]
    pub property: crate::datadogV2::model::FlagSuggestionProperty,
    /// The suggested new value (JSON-encoded for complex properties, empty string for flag-level actions like archive).
    #[serde(rename = "suggestion")]
    pub suggestion: Option<String>,
    /// Optional metadata for a suggestion.
    #[serde(rename = "suggestion_metadata")]
    pub suggestion_metadata: Option<crate::datadogV2::model::SuggestionMetadata>,
    /// When the suggestion was last updated.
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// UUID of the user who last updated the suggestion.
    #[serde(
        rename = "updated_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_by: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlagSuggestionAttributes {
    pub fn new(
        action: crate::datadogV2::model::FlagSuggestionAction,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: uuid::Uuid,
        current_status: crate::datadogV2::model::FlagSuggestionStatus,
        feature_flag_id: uuid::Uuid,
        property: crate::datadogV2::model::FlagSuggestionProperty,
    ) -> FlagSuggestionAttributes {
        FlagSuggestionAttributes {
            action,
            base_flag_history_id: None,
            comment: None,
            created_at,
            created_by,
            current_status,
            current_value: None,
            deleted_at: None,
            deleted_by: None,
            environment_id: None,
            feature_flag_id,
            message: None,
            property,
            suggestion: None,
            suggestion_metadata: None,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn base_flag_history_id(mut self, value: uuid::Uuid) -> Self {
        self.base_flag_history_id = Some(value);
        self
    }

    pub fn comment(mut self, value: Option<String>) -> Self {
        self.comment = Some(value);
        self
    }

    pub fn current_value(mut self, value: String) -> Self {
        self.current_value = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn deleted_by(mut self, value: Option<String>) -> Self {
        self.deleted_by = Some(value);
        self
    }

    pub fn environment_id(mut self, value: Option<String>) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn suggestion(mut self, value: String) -> Self {
        self.suggestion = Some(value);
        self
    }

    pub fn suggestion_metadata(
        mut self,
        value: crate::datadogV2::model::SuggestionMetadata,
    ) -> Self {
        self.suggestion_metadata = Some(value);
        self
    }

    pub fn updated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: Option<String>) -> Self {
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

impl<'de> Deserialize<'de> for FlagSuggestionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlagSuggestionAttributesVisitor;
        impl<'a> Visitor<'a> for FlagSuggestionAttributesVisitor {
            type Value = FlagSuggestionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::FlagSuggestionAction> = None;
                let mut base_flag_history_id: Option<uuid::Uuid> = None;
                let mut comment: Option<Option<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut current_status: Option<crate::datadogV2::model::FlagSuggestionStatus> =
                    None;
                let mut current_value: Option<String> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut deleted_by: Option<Option<String>> = None;
                let mut environment_id: Option<Option<String>> = None;
                let mut feature_flag_id: Option<uuid::Uuid> = None;
                let mut message: Option<String> = None;
                let mut property: Option<crate::datadogV2::model::FlagSuggestionProperty> = None;
                let mut suggestion: Option<String> = None;
                let mut suggestion_metadata: Option<crate::datadogV2::model::SuggestionMetadata> =
                    None;
                let mut updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut updated_by: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::FlagSuggestionAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "base_flag_history_id" => {
                            if v.is_null() {
                                continue;
                            }
                            base_flag_history_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "comment" => {
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
                                    crate::datadogV2::model::FlagSuggestionStatus::UnparsedObject(_current_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "current_value" => {
                            if v.is_null() {
                                continue;
                            }
                            current_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_by" => {
                            deleted_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
                            feature_flag_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _property) = property {
                                match _property {
                                    crate::datadogV2::model::FlagSuggestionProperty::UnparsedObject(_property) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "suggestion" => {
                            if v.is_null() {
                                continue;
                            }
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
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let current_status =
                    current_status.ok_or_else(|| M::Error::missing_field("current_status"))?;
                let feature_flag_id =
                    feature_flag_id.ok_or_else(|| M::Error::missing_field("feature_flag_id"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = FlagSuggestionAttributes {
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

        deserializer.deserialize_any(FlagSuggestionAttributesVisitor)
    }
}
