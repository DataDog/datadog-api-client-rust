// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Project relationships
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectRelationships {
    /// Relationship between a team and a team link
    #[serde(rename = "member_team")]
    pub member_team: Option<crate::datadogV2::model::RelationshipToTeamLinks>,
    /// Relationship to users.
    #[serde(rename = "member_user")]
    pub member_user: Option<crate::datadogV2::model::UsersRelationship>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectRelationships {
    pub fn new() -> ProjectRelationships {
        ProjectRelationships {
            member_team: None,
            member_user: None,
            _unparsed: false,
        }
    }

    pub fn member_team(mut self, value: crate::datadogV2::model::RelationshipToTeamLinks) -> Self {
        self.member_team = Some(value);
        self
    }

    pub fn member_user(mut self, value: crate::datadogV2::model::UsersRelationship) -> Self {
        self.member_user = Some(value);
        self
    }
}

impl Default for ProjectRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectRelationshipsVisitor;
        impl<'a> Visitor<'a> for ProjectRelationshipsVisitor {
            type Value = ProjectRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut member_team: Option<crate::datadogV2::model::RelationshipToTeamLinks> =
                    None;
                let mut member_user: Option<crate::datadogV2::model::UsersRelationship> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "member_team" => {
                            if v.is_null() {
                                continue;
                            }
                            member_team =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "member_user" => {
                            if v.is_null() {
                                continue;
                            }
                            member_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ProjectRelationships {
                    member_team,
                    member_user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectRelationshipsVisitor)
    }
}
