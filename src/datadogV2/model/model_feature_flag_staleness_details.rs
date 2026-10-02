// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The feature flag's current staleness state and suggested actions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagStalenessDetails {
    /// Repositories and files where the flag is referenced in source code.
    #[serde(
        rename = "code_references",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub code_references:
        Option<Option<Vec<crate::datadogV2::model::FeatureFlagStalenessCodeReference>>>,
    /// The ID of the user who dismissed the staleness recommendation.
    #[serde(
        rename = "dismissed_by",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dismissed_by: Option<Option<String>>,
    /// The ID of the feature flag whose staleness state is returned.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Suggested actions for the flag. The first action is the primary recommendation.
    #[serde(
        rename = "recommended_actions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub recommended_actions:
        Option<Option<Vec<crate::datadogV2::model::FeatureFlagStalenessRecommendedAction>>>,
    /// Time until which staleness checks are paused for the flag.
    #[serde(
        rename = "skip_state_check_until",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub skip_state_check_until: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Why the flag is stale or has a manually selected state. Values include `FULLY_ROLLED_OUT`, `NO_EVALUATIONS`, `NO_ACTIVITY`, and `USER_SET`.
    #[serde(
        rename = "stale_reason",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stale_reason: Option<Option<String>>,
    /// The current state, such as `ACTIVE`, `STALE`, or `PERMANENT`.
    #[serde(rename = "staleness_status")]
    pub staleness_status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagStalenessDetails {
    pub fn new() -> FeatureFlagStalenessDetails {
        FeatureFlagStalenessDetails {
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
        value: Option<Vec<crate::datadogV2::model::FeatureFlagStalenessCodeReference>>,
    ) -> Self {
        self.code_references = Some(value);
        self
    }

    pub fn dismissed_by(mut self, value: Option<String>) -> Self {
        self.dismissed_by = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn recommended_actions(
        mut self,
        value: Option<Vec<crate::datadogV2::model::FeatureFlagStalenessRecommendedAction>>,
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

impl Default for FeatureFlagStalenessDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FeatureFlagStalenessDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagStalenessDetailsVisitor;
        impl<'a> Visitor<'a> for FeatureFlagStalenessDetailsVisitor {
            type Value = FeatureFlagStalenessDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code_references: Option<
                    Option<Vec<crate::datadogV2::model::FeatureFlagStalenessCodeReference>>,
                > = None;
                let mut dismissed_by: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut recommended_actions: Option<
                    Option<Vec<crate::datadogV2::model::FeatureFlagStalenessRecommendedAction>>,
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

                let content = FeatureFlagStalenessDetails {
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

        deserializer.deserialize_any(FeatureFlagStalenessDetailsVisitor)
    }
}
