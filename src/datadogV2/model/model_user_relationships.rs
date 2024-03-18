// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the user object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UserRelationships {
    /// Relationship to roles.
    #[serde(rename = "roles")]
    pub roles: Option<crate::datadogV2::model::RelationshipToRoles>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UserRelationships {
    pub fn new() -> UserRelationships {
        UserRelationships {
            roles: None,
            _unparsed: false,
        }
    }

    pub fn roles(mut self, value: crate::datadogV2::model::RelationshipToRoles) -> Self {
        self.roles = Some(value);
        self
    }
}

impl Default for UserRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UserRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserRelationshipsVisitor;
        impl<'a> Visitor<'a> for UserRelationshipsVisitor {
            type Value = UserRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut roles: Option<crate::datadogV2::model::RelationshipToRoles> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "roles" => {
                            if v.is_null() {
                                continue;
                            }
                            roles = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UserRelationships { roles, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UserRelationshipsVisitor)
    }
}
