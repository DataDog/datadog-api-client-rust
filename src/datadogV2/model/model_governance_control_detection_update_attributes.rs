// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance control detection that can be updated. Only the attributes present in the request are modified.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlDetectionUpdateAttributes {
    /// The handle of the team the detection is assigned to. Set to an empty string to clear the assignment.
    #[serde(rename = "assigned_team")]
    pub assigned_team: Option<String>,
    /// The UUID of the user the detection is assigned to. Set to an empty string to clear the assignment.
    #[serde(rename = "assigned_to")]
    pub assigned_to: Option<String>,
    /// The timestamp after which the detection becomes eligible for mitigation. Used to defer mitigation to a later time.
    #[serde(rename = "mitigate_after")]
    pub mitigate_after: Option<chrono::DateTime<chrono::Utc>>,
    /// The new state to set for the detection. Set to `exception` to acknowledge the detection and exclude it from active counts, or `active` to reopen it.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::GovernanceControlDetectionUpdateState>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlDetectionUpdateAttributes {
    pub fn new() -> GovernanceControlDetectionUpdateAttributes {
        GovernanceControlDetectionUpdateAttributes {
            assigned_team: None,
            assigned_to: None,
            mitigate_after: None,
            state: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assigned_team(mut self, value: String) -> Self {
        self.assigned_team = Some(value);
        self
    }

    pub fn assigned_to(mut self, value: String) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn mitigate_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.mitigate_after = Some(value);
        self
    }

    pub fn state(
        mut self,
        value: crate::datadogV2::model::GovernanceControlDetectionUpdateState,
    ) -> Self {
        self.state = Some(value);
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

impl Default for GovernanceControlDetectionUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GovernanceControlDetectionUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlDetectionUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceControlDetectionUpdateAttributesVisitor {
            type Value = GovernanceControlDetectionUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assigned_team: Option<String> = None;
                let mut assigned_to: Option<String> = None;
                let mut mitigate_after: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut state: Option<
                    crate::datadogV2::model::GovernanceControlDetectionUpdateState,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assigned_team" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_team =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assigned_to" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_to =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigate_after" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigate_after =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::GovernanceControlDetectionUpdateState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GovernanceControlDetectionUpdateAttributes {
                    assigned_team,
                    assigned_to,
                    mitigate_after,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlDetectionUpdateAttributesVisitor)
    }
}
