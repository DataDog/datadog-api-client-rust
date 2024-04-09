// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship of AuthN Mapping to a Team.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingRelationshipToTeam {
    /// Relationship to team.
    #[serde(rename = "team")]
    pub team: crate::datadogV2::model::RelationshipToTeam,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingRelationshipToTeam {
    pub fn new(
        team: crate::datadogV2::model::RelationshipToTeam,
    ) -> AuthNMappingRelationshipToTeam {
        AuthNMappingRelationshipToTeam {
            team,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AuthNMappingRelationshipToTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingRelationshipToTeamVisitor;
        impl<'a> Visitor<'a> for AuthNMappingRelationshipToTeamVisitor {
            type Value = AuthNMappingRelationshipToTeam;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut team: Option<crate::datadogV2::model::RelationshipToTeam> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "team" => {
                            team = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let team = team.ok_or_else(|| M::Error::missing_field("team"))?;

                let content = AuthNMappingRelationshipToTeam { team, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingRelationshipToTeamVisitor)
    }
}
