// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the team connection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamConnectionRelationships {
    /// Reference to a team from an external system.
    #[serde(rename = "connected_team")]
    pub connected_team: Option<crate::datadogV2::model::ConnectedTeamRef>,
    /// Reference to a Datadog team.
    #[serde(rename = "team")]
    pub team: Option<crate::datadogV2::model::TeamRef>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamConnectionRelationships {
    pub fn new() -> TeamConnectionRelationships {
        TeamConnectionRelationships {
            connected_team: None,
            team: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn connected_team(mut self, value: crate::datadogV2::model::ConnectedTeamRef) -> Self {
        self.connected_team = Some(value);
        self
    }

    pub fn team(mut self, value: crate::datadogV2::model::TeamRef) -> Self {
        self.team = Some(value);
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

impl Default for TeamConnectionRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamConnectionRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamConnectionRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamConnectionRelationshipsVisitor {
            type Value = TeamConnectionRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connected_team: Option<crate::datadogV2::model::ConnectedTeamRef> = None;
                let mut team: Option<crate::datadogV2::model::TeamRef> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connected_team" => {
                            if v.is_null() {
                                continue;
                            }
                            connected_team =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamConnectionRelationships {
                    connected_team,
                    team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamConnectionRelationshipsVisitor)
    }
}
