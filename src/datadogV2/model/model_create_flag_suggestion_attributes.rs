// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a flag suggestion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateFlagSuggestionAttributes {
    /// The type of change action for a suggestion.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::FlagSuggestionAction,
    /// Optional comment explaining the change.
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// The environment ID for environment-scoped changes.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// Notification handles (without @ prefix) to receive approval or rejection notifications.
    /// For example, an email address or Slack channel name.
    #[serde(rename = "notification_rule_targets")]
    pub notification_rule_targets: Vec<String>,
    /// The flag property being changed.
    #[serde(rename = "property")]
    pub property: crate::datadogV2::model::FlagSuggestionProperty,
    /// The suggested new value (empty string for flag-level actions like archive, JSON-encoded for complex properties like allocations).
    #[serde(rename = "suggestion")]
    pub suggestion: Option<String>,
    /// Optional metadata for a suggestion.
    #[serde(rename = "suggestion_metadata")]
    pub suggestion_metadata: Option<crate::datadogV2::model::SuggestionMetadata>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateFlagSuggestionAttributes {
    pub fn new(
        action: crate::datadogV2::model::FlagSuggestionAction,
        notification_rule_targets: Vec<String>,
        property: crate::datadogV2::model::FlagSuggestionProperty,
    ) -> CreateFlagSuggestionAttributes {
        CreateFlagSuggestionAttributes {
            action,
            comment: None,
            environment_id: None,
            notification_rule_targets,
            property,
            suggestion: None,
            suggestion_metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn comment(mut self, value: String) -> Self {
        self.comment = Some(value);
        self
    }

    pub fn environment_id(mut self, value: uuid::Uuid) -> Self {
        self.environment_id = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CreateFlagSuggestionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateFlagSuggestionAttributesVisitor;
        impl<'a> Visitor<'a> for CreateFlagSuggestionAttributesVisitor {
            type Value = CreateFlagSuggestionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::FlagSuggestionAction> = None;
                let mut comment: Option<String> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut notification_rule_targets: Option<Vec<String>> = None;
                let mut property: Option<crate::datadogV2::model::FlagSuggestionProperty> = None;
                let mut suggestion: Option<String> = None;
                let mut suggestion_metadata: Option<crate::datadogV2::model::SuggestionMetadata> =
                    None;
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
                        "comment" => {
                            if v.is_null() {
                                continue;
                            }
                            comment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_rule_targets" => {
                            notification_rule_targets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let notification_rule_targets = notification_rule_targets
                    .ok_or_else(|| M::Error::missing_field("notification_rule_targets"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;

                let content = CreateFlagSuggestionAttributes {
                    action,
                    comment,
                    environment_id,
                    notification_rule_targets,
                    property,
                    suggestion,
                    suggestion_metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateFlagSuggestionAttributesVisitor)
    }
}
