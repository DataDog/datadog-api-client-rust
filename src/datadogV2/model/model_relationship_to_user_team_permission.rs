// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship between a user team permission and a team
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationshipToUserTeamPermission {
    /// Related user team permission data
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::RelationshipToUserTeamPermissionData>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::TeamRelationshipsLinks>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RelationshipToUserTeamPermission {
    pub fn new() -> RelationshipToUserTeamPermission {
        RelationshipToUserTeamPermission {
            data: None,
            links: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: crate::datadogV2::model::RelationshipToUserTeamPermissionData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(mut self, value: crate::datadogV2::model::TeamRelationshipsLinks) -> Self {
        self.links = Some(value);
        self
    }
}

impl Default for RelationshipToUserTeamPermission {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RelationshipToUserTeamPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RelationshipToUserTeamPermissionVisitor;
        impl<'a> Visitor<'a> for RelationshipToUserTeamPermissionVisitor {
            type Value = RelationshipToUserTeamPermission;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::RelationshipToUserTeamPermissionData,
                > = None;
                let mut links: Option<crate::datadogV2::model::TeamRelationshipsLinks> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = RelationshipToUserTeamPermission {
                    data,
                    links,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RelationshipToUserTeamPermissionVisitor)
    }
}
