// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resources related to a team
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamRelationships {
    /// Relationship between a team and a team link
    #[serde(rename = "team_links")]
    pub team_links: Option<crate::datadogV2::model::RelationshipToTeamLinks>,
    /// Relationship between a user team permission and a team
    #[serde(rename = "user_team_permissions")]
    pub user_team_permissions: Option<crate::datadogV2::model::RelationshipToUserTeamPermission>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamRelationships {
    pub fn new() -> TeamRelationships {
        TeamRelationships {
            team_links: None,
            user_team_permissions: None,
            _unparsed: false,
        }
    }

    pub fn team_links(mut self, value: crate::datadogV2::model::RelationshipToTeamLinks) -> Self {
        self.team_links = Some(value);
        self
    }

    pub fn user_team_permissions(
        mut self,
        value: crate::datadogV2::model::RelationshipToUserTeamPermission,
    ) -> Self {
        self.user_team_permissions = Some(value);
        self
    }
}

impl Default for TeamRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamRelationshipsVisitor {
            type Value = TeamRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut team_links: Option<crate::datadogV2::model::RelationshipToTeamLinks> = None;
                let mut user_team_permissions: Option<
                    crate::datadogV2::model::RelationshipToUserTeamPermission,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "team_links" => {
                            if v.is_null() {
                                continue;
                            }
                            team_links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_team_permissions" => {
                            if v.is_null() {
                                continue;
                            }
                            user_team_permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TeamRelationships {
                    team_links,
                    user_team_permissions,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamRelationshipsVisitor)
    }
}
