// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Groups the relationships for a schedule object, referencing layers and teams.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleDataRelationships {
    /// Associates layers with this schedule in a data structure.
    #[serde(rename = "layers")]
    pub layers: Option<crate::datadogV2::model::ScheduleDataRelationshipsLayers>,
    /// Associates teams with this schedule in a data structure.
    #[serde(rename = "teams")]
    pub teams: Option<crate::datadogV2::model::ScheduleDataRelationshipsTeams>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleDataRelationships {
    pub fn new() -> ScheduleDataRelationships {
        ScheduleDataRelationships {
            layers: None,
            teams: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn layers(
        mut self,
        value: crate::datadogV2::model::ScheduleDataRelationshipsLayers,
    ) -> Self {
        self.layers = Some(value);
        self
    }

    pub fn teams(mut self, value: crate::datadogV2::model::ScheduleDataRelationshipsTeams) -> Self {
        self.teams = Some(value);
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

impl Default for ScheduleDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScheduleDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for ScheduleDataRelationshipsVisitor {
            type Value = ScheduleDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut layers: Option<crate::datadogV2::model::ScheduleDataRelationshipsLayers> =
                    None;
                let mut teams: Option<crate::datadogV2::model::ScheduleDataRelationshipsTeams> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "layers" => {
                            if v.is_null() {
                                continue;
                            }
                            layers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScheduleDataRelationships {
                    layers,
                    teams,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScheduleDataRelationshipsVisitor)
    }
}
