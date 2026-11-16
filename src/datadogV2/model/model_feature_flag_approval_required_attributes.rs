// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The change to submit for approval.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagApprovalRequiredAttributes {
    /// The proposed action.
    #[serde(rename = "action")]
    pub action: String,
    /// The ID of the environment targeted by the change.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// The feature flag property to change.
    #[serde(rename = "property")]
    pub property: String,
    /// The proposed value, serialized as a string. Allocation sets are JSON-encoded strings.
    #[serde(rename = "suggestion")]
    pub suggestion: String,
    /// Metadata associated with a suggestion. Fields other than `variant_id` are set by the server.
    #[serde(rename = "suggestion_metadata")]
    pub suggestion_metadata: Option<crate::datadogV2::model::FeatureFlagSuggestionMetadata>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagApprovalRequiredAttributes {
    pub fn new(
        action: String,
        property: String,
        suggestion: String,
    ) -> FeatureFlagApprovalRequiredAttributes {
        FeatureFlagApprovalRequiredAttributes {
            action,
            environment_id: None,
            property,
            suggestion,
            suggestion_metadata: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn environment_id(mut self, value: uuid::Uuid) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn suggestion_metadata(
        mut self,
        value: crate::datadogV2::model::FeatureFlagSuggestionMetadata,
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

impl<'de> Deserialize<'de> for FeatureFlagApprovalRequiredAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagApprovalRequiredAttributesVisitor;
        impl<'a> Visitor<'a> for FeatureFlagApprovalRequiredAttributesVisitor {
            type Value = FeatureFlagApprovalRequiredAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut property: Option<String> = None;
                let mut suggestion: Option<String> = None;
                let mut suggestion_metadata: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionMetadata,
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
                        "environment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let property = property.ok_or_else(|| M::Error::missing_field("property"))?;
                let suggestion = suggestion.ok_or_else(|| M::Error::missing_field("suggestion"))?;

                let content = FeatureFlagApprovalRequiredAttributes {
                    action,
                    environment_id,
                    property,
                    suggestion,
                    suggestion_metadata,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagApprovalRequiredAttributesVisitor)
    }
}
