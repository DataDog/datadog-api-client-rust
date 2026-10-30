// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a feature flag change event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagChangeEventAttributes {
    /// The kind of change.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::FeatureFlagChangeEventAction,
    /// Additional details about the change, as a JSON-encoded string.
    #[serde(
        rename = "change_metadata",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub change_metadata: Option<Option<String>>,
    /// Where the change was made: `UI` for the Datadog app, or `MCP` for a Model Context Protocol (MCP) client.
    #[serde(
        rename = "creation_source",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub creation_source:
        Option<Option<crate::datadogV2::model::FeatureFlagChangeEventCreationSource>>,
    /// A readable sentence describing the change, as shown on the flag history page and in the Action column of the CSV export.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The environment the change applies to. Null for a change to the flag itself rather than to one environment.
    #[serde(
        rename = "environment",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub environment: Option<Option<crate::datadogV2::model::FeatureFlagChangeEventEnvironment>>,
    /// The feature flag the change applies to.
    #[serde(rename = "feature_flag")]
    pub feature_flag: crate::datadogV2::model::FeatureFlagChangeEventFlag,
    /// The value after the change, serialized as a string.
    #[serde(
        rename = "new_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub new_value: Option<Option<String>>,
    /// The value before the change, serialized as a string.
    #[serde(
        rename = "previous_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub previous_value: Option<Option<String>>,
    /// The part of the feature flag the change applies to.
    #[serde(rename = "property")]
    pub property: crate::datadogV2::model::FeatureFlagChangeEventProperty,
    /// Review details of a change request that needed approval.
    /// Present on review events, which `include_suggestion_events` adds.
    /// Also present on a change that an approved change request applied, even when `include_suggestion_events` is `false`.
    #[serde(rename = "suggestion_review")]
    pub suggestion_review: Option<crate::datadogV2::model::FeatureFlagChangeEventSuggestionReview>,
    /// When the change was made.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// The ID of the user who made the change. Null when no user made the change.
    #[serde(
        rename = "user_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<String>>,
    /// The version of the feature flag the change produced.
    #[serde(rename = "version_number")]
    pub version_number: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagChangeEventAttributes {
    pub fn new(
        action: crate::datadogV2::model::FeatureFlagChangeEventAction,
        feature_flag: crate::datadogV2::model::FeatureFlagChangeEventFlag,
        property: crate::datadogV2::model::FeatureFlagChangeEventProperty,
        timestamp: chrono::DateTime<chrono::Utc>,
        version_number: i64,
    ) -> FeatureFlagChangeEventAttributes {
        FeatureFlagChangeEventAttributes {
            action,
            change_metadata: None,
            creation_source: None,
            description: None,
            environment: None,
            feature_flag,
            new_value: None,
            previous_value: None,
            property,
            suggestion_review: None,
            timestamp,
            user_id: None,
            version_number,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_metadata(mut self, value: Option<String>) -> Self {
        self.change_metadata = Some(value);
        self
    }

    pub fn creation_source(
        mut self,
        value: Option<crate::datadogV2::model::FeatureFlagChangeEventCreationSource>,
    ) -> Self {
        self.creation_source = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn environment(
        mut self,
        value: Option<crate::datadogV2::model::FeatureFlagChangeEventEnvironment>,
    ) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn new_value(mut self, value: Option<String>) -> Self {
        self.new_value = Some(value);
        self
    }

    pub fn previous_value(mut self, value: Option<String>) -> Self {
        self.previous_value = Some(value);
        self
    }

    pub fn suggestion_review(
        mut self,
        value: crate::datadogV2::model::FeatureFlagChangeEventSuggestionReview,
    ) -> Self {
        self.suggestion_review = Some(value);
        self
    }

    pub fn user_id(mut self, value: Option<String>) -> Self {
        self.user_id = Some(value);
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

impl<'de> Deserialize<'de> for FeatureFlagChangeEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagChangeEventAttributesVisitor;
        impl<'a> Visitor<'a> for FeatureFlagChangeEventAttributesVisitor {
            type Value = FeatureFlagChangeEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::FeatureFlagChangeEventAction> =
                    None;
                let mut change_metadata: Option<Option<String>> = None;
                let mut creation_source: Option<
                    Option<crate::datadogV2::model::FeatureFlagChangeEventCreationSource>,
                > = None;
                let mut description: Option<String> = None;
                let mut environment: Option<
                    Option<crate::datadogV2::model::FeatureFlagChangeEventEnvironment>,
                > = None;
                let mut feature_flag: Option<crate::datadogV2::model::FeatureFlagChangeEventFlag> =
                    None;
                let mut new_value: Option<Option<String>> = None;
                let mut previous_value: Option<Option<String>> = None;
                let mut property: Option<crate::datadogV2::model::FeatureFlagChangeEventProperty> =
                    None;
                let mut suggestion_review: Option<
                    crate::datadogV2::model::FeatureFlagChangeEventSuggestionReview,
                > = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut user_id: Option<Option<String>> = None;
                let mut version_number: Option<i64> = None;
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
                                    crate::datadogV2::model::FeatureFlagChangeEventAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "change_metadata" => {
                            change_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creation_source" => {
                            creation_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _creation_source) = creation_source {
                                match _creation_source {
                                    Some(crate::datadogV2::model::FeatureFlagChangeEventCreationSource::UnparsedObject(_creation_source)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment" => {
                            environment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag" => {
                            feature_flag =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_value" => {
                            new_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous_value" => {
                            previous_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property" => {
                            property = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _property) = property {
                                match _property {
                                    crate::datadogV2::model::FeatureFlagChangeEventProperty::UnparsedObject(_property) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "suggestion_review" => {
                            if v.is_null() {
                                continue;
                            }
                            suggestion_review =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_number" => {
                            version_number =
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
                let feature_flag =
                    feature_flag.ok_or_else(|| M::Error::missing_field("feature_flag"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;
                let version_number =
                    version_number.ok_or_else(|| M::Error::missing_field("version_number"))?;

                let content = FeatureFlagChangeEventAttributes {
                    action,
                    change_metadata,
                    creation_source,
                    description,
                    environment,
                    feature_flag,
                    new_value,
                    previous_value,
                    property,
                    suggestion_review,
                    timestamp,
                    user_id,
                    version_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagChangeEventAttributesVisitor)
    }
}
