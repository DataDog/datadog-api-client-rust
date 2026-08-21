// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual team's ownership entry within a teams ownership rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsOwnershipRuleTeamMapping {
    /// The ID of the underlying mapping, used to delete this team's ownership individually.
    #[serde(rename = "mapping_id")]
    pub mapping_id: String,
    /// The handle of the owning team.
    #[serde(rename = "team_handle")]
    pub team_handle: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamsOwnershipRuleTeamMapping {
    pub fn new(mapping_id: String, team_handle: String) -> TeamsOwnershipRuleTeamMapping {
        TeamsOwnershipRuleTeamMapping {
            mapping_id,
            team_handle,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TeamsOwnershipRuleTeamMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamsOwnershipRuleTeamMappingVisitor;
        impl<'a> Visitor<'a> for TeamsOwnershipRuleTeamMappingVisitor {
            type Value = TeamsOwnershipRuleTeamMapping;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mapping_id: Option<String> = None;
                let mut team_handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mapping_id" => {
                            mapping_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_handle" => {
                            team_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mapping_id = mapping_id.ok_or_else(|| M::Error::missing_field("mapping_id"))?;
                let team_handle =
                    team_handle.ok_or_else(|| M::Error::missing_field("team_handle"))?;

                let content = TeamsOwnershipRuleTeamMapping {
                    mapping_id,
                    team_handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamsOwnershipRuleTeamMappingVisitor)
    }
}
