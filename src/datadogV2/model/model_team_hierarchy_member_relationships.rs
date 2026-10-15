// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User and requested team linkage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyMemberRelationships {
    /// Linkage to the requested team.
    #[serde(rename = "team")]
    pub team: crate::datadogV2::model::TeamHierarchyMemberTeamRelationship,
    /// Linkage to the user.
    #[serde(rename = "user")]
    pub user: crate::datadogV2::model::TeamHierarchyMemberUserRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyMemberRelationships {
    pub fn new(
        team: crate::datadogV2::model::TeamHierarchyMemberTeamRelationship,
        user: crate::datadogV2::model::TeamHierarchyMemberUserRelationship,
    ) -> TeamHierarchyMemberRelationships {
        TeamHierarchyMemberRelationships {
            team,
            user,
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

impl<'de> Deserialize<'de> for TeamHierarchyMemberRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyMemberRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyMemberRelationshipsVisitor {
            type Value = TeamHierarchyMemberRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut team: Option<crate::datadogV2::model::TeamHierarchyMemberTeamRelationship> =
                    None;
                let mut user: Option<crate::datadogV2::model::TeamHierarchyMemberUserRelationship> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "team" => {
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let team = team.ok_or_else(|| M::Error::missing_field("team"))?;
                let user = user.ok_or_else(|| M::Error::missing_field("user"))?;

                let content = TeamHierarchyMemberRelationships {
                    team,
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyMemberRelationshipsVisitor)
    }
}
