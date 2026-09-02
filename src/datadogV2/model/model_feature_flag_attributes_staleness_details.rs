// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details about the feature flag's staleness status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagAttributesStalenessDetails {
    /// Code references associated with the feature flag.
    #[serde(
        rename = "code_references",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub code_references: Option<Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>>,
    /// The ID of the user who dismissed the staleness notification.
    #[serde(
        rename = "dismissed_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dismissed_by: Option<Option<uuid::Uuid>>,
    /// The unique identifier of the staleness details record.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// Recommended actions to address the feature flag's staleness.
    #[serde(
        rename = "recommended_actions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub recommended_actions:
        Option<Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>>,
    /// The timestamp until which staleness checks are skipped.
    #[serde(
        rename = "skip_state_check_until",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub skip_state_check_until: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The reason the feature flag is considered stale.
    #[serde(
        rename = "stale_reason",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stale_reason: Option<Option<String>>,
    /// The staleness status of the feature flag.
    #[serde(rename = "staleness_status")]
    pub staleness_status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagAttributesStalenessDetails {
    pub fn new() -> FeatureFlagAttributesStalenessDetails {
        FeatureFlagAttributesStalenessDetails {
            code_references: None,
            dismissed_by: None,
            id: None,
            recommended_actions: None,
            skip_state_check_until: None,
            stale_reason: None,
            staleness_status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn code_references(
        mut self,
        value: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    ) -> Self {
        self.code_references = Some(value);
        self
    }

    pub fn dismissed_by(mut self, value: Option<uuid::Uuid>) -> Self {
        self.dismissed_by = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    pub fn recommended_actions(
        mut self,
        value: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    ) -> Self {
        self.recommended_actions = Some(value);
        self
    }

    pub fn skip_state_check_until(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.skip_state_check_until = Some(value);
        self
    }

    pub fn stale_reason(mut self, value: Option<String>) -> Self {
        self.stale_reason = Some(value);
        self
    }

    pub fn staleness_status(mut self, value: String) -> Self {
        self.staleness_status = Some(value);
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

impl Default for FeatureFlagAttributesStalenessDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FeatureFlagAttributesStalenessDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagAttributesStalenessDetailsVisitor;
        impl<'a> Visitor<'a> for FeatureFlagAttributesStalenessDetailsVisitor {
            type Value = FeatureFlagAttributesStalenessDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_references: Option<
                    Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
                > = None;
                let mut dismissed_by: Option<Option<uuid::Uuid>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut recommended_actions: Option<
                    Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
                > = None;
                let mut skip_state_check_until: Option<Option<chrono::DateTime<chrono::Utc>>> =
                    None;
                let mut stale_reason: Option<Option<String>> = None;
                let mut staleness_status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code_references" => {
                            code_references =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dismissed_by" => {
                            dismissed_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended_actions" => {
                            recommended_actions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "skip_state_check_until" => {
                            skip_state_check_until =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stale_reason" => {
                            stale_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "staleness_status" => {
                            if v.is_null() {
                                continue;
                            }
                            staleness_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FeatureFlagAttributesStalenessDetails {
                    code_references,
                    dismissed_by,
                    id,
                    recommended_actions,
                    skip_state_check_until,
                    stale_reason,
                    staleness_status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagAttributesStalenessDetailsVisitor)
    }
}
