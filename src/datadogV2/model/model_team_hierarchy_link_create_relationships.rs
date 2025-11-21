// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team hierarchy link creation relationships
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamHierarchyLinkCreateRelationships {
    /// Team relationship for hierarchy link creation
    #[serde(rename = "parent_team")]
    pub parent_team: crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
    /// Team relationship for hierarchy link creation
    #[serde(rename = "sub_team")]
    pub sub_team: crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamHierarchyLinkCreateRelationships {
    pub fn new(
        parent_team: crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
        sub_team: crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
    ) -> TeamHierarchyLinkCreateRelationships {
        TeamHierarchyLinkCreateRelationships {
            parent_team,
            sub_team,
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

impl<'de> Deserialize<'de> for TeamHierarchyLinkCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamHierarchyLinkCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamHierarchyLinkCreateRelationshipsVisitor {
            type Value = TeamHierarchyLinkCreateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut parent_team: Option<
                    crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
                > = None;
                let mut sub_team: Option<
                    crate::datadogV2::model::TeamHierarchyLinkCreateTeamRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "parent_team" => {
                            parent_team =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_team" => {
                            sub_team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let parent_team =
                    parent_team.ok_or_else(|| M::Error::missing_field("parent_team"))?;
                let sub_team = sub_team.ok_or_else(|| M::Error::missing_field("sub_team"))?;

                let content = TeamHierarchyLinkCreateRelationships {
                    parent_team,
                    sub_team,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamHierarchyLinkCreateRelationshipsVisitor)
    }
}
