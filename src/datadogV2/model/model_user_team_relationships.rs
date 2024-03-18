// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship between membership and a user
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserTeamRelationships {
    /// Relationship between team membership and team
    #[serde(rename = "team")]
    pub team: Option<crate::datadogV2::model::RelationshipToUserTeamTeam>,
    /// Relationship between team membership and user
    #[serde(rename = "user")]
    pub user: Option<crate::datadogV2::model::RelationshipToUserTeamUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserTeamRelationships {
    pub fn new() -> UserTeamRelationships {
        UserTeamRelationships {
            team: None,
            user: None,
            _unparsed: false,
        }
    }

    pub fn team(mut self, value: crate::datadogV2::model::RelationshipToUserTeamTeam) -> Self {
        self.team = Some(value);
        self
    }

    pub fn user(mut self, value: crate::datadogV2::model::RelationshipToUserTeamUser) -> Self {
        self.user = Some(value);
        self
    }
}

impl Default for UserTeamRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserTeamRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserTeamRelationshipsVisitor;
        impl<'a> Visitor<'a> for UserTeamRelationshipsVisitor {
            type Value = UserTeamRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut team: Option<crate::datadogV2::model::RelationshipToUserTeamTeam> = None;
                let mut user: Option<crate::datadogV2::model::RelationshipToUserTeamUser> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "team" => {
                            if v.is_null() {
                                continue;
                            }
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserTeamRelationships {
                    team,
                    user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserTeamRelationshipsVisitor)
    }
}
