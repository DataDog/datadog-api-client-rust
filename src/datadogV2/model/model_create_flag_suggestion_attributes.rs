// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a feature flag change suggestion.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateFlagSuggestionAttributes {
    /// The action proposed by the suggestion.
    #[serde(rename = "action")]
    pub action: String,
    /// A comment explaining the proposed change.
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// The ID of the target environment. Required for `ALLOCATIONS`, `DEFAULT_VARIANT`, and
    /// `OVERRIDE_VARIANT` changes.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// Handles to notify about the review, without the `@` prefix. For example, an email address
    /// or a Slack channel name.
    #[serde(rename = "notification_rule_targets")]
    pub notification_rule_targets: Vec<String>,
    /// The property to change. The property must require approval for this flag and environment.
    #[serde(rename = "property")]
    pub property: String,
    /// The proposed value as a string. For structured values, supply a JSON-encoded string.
    #[serde(rename = "suggestion")]
    pub suggestion: Option<String>,
    /// Client-settable metadata for a suggestion.
    #[serde(rename = "suggestion_metadata")]
    pub suggestion_metadata: Option<crate::datadogV2::model::FeatureFlagSuggestionMetadataRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateFlagSuggestionAttributes {
    pub fn new(
        action: String,
        notification_rule_targets: Vec<String>,
        property: String,
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
        value: crate::datadogV2::model::FeatureFlagSuggestionMetadataRequest,
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
                let mut action: Option<String> = None;
                let mut comment: Option<String> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut notification_rule_targets: Option<Vec<String>> = None;
                let mut property: Option<String> = None;
                let mut suggestion: Option<String> = None;
                let mut suggestion_metadata: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionMetadataRequest,
                > = None;
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
