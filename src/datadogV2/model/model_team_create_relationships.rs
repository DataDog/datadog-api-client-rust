// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships formed with the team on creation
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamCreateRelationships {
    /// Relationship to users.
    #[serde(rename = "users")]
    pub users: Option<crate::datadogV2::model::RelationshipToUsers>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamCreateRelationships {
    pub fn new() -> TeamCreateRelationships {
        TeamCreateRelationships {
            users: None,
            _unparsed: false,
        }
    }

    pub fn users(mut self, value: crate::datadogV2::model::RelationshipToUsers) -> Self {
        self.users = Some(value);
        self
    }
}

impl Default for TeamCreateRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamCreateRelationshipsVisitor;
        impl<'a> Visitor<'a> for TeamCreateRelationshipsVisitor {
            type Value = TeamCreateRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut users: Option<crate::datadogV2::model::RelationshipToUsers> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TeamCreateRelationships { users, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamCreateRelationshipsVisitor)
    }
}
