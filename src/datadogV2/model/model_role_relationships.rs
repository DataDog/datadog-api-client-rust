// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the role object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoleRelationships {
    /// Relationship to multiple permissions objects.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::RelationshipToPermissions>,
    /// Relationship to users.
    #[serde(rename = "users")]
    pub users: Option<crate::datadogV2::model::RelationshipToUsers>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoleRelationships {
    pub fn new() -> RoleRelationships {
        RoleRelationships {
            permissions: None,
            users: None,
            _unparsed: false,
        }
    }

    pub fn permissions(
        &mut self,
        value: crate::datadogV2::model::RelationshipToPermissions,
    ) -> &mut Self {
        self.permissions = Some(value);
        self
    }

    pub fn users(&mut self, value: crate::datadogV2::model::RelationshipToUsers) -> &mut Self {
        self.users = Some(value);
        self
    }
}

impl Default for RoleRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RoleRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoleRelationshipsVisitor;
        impl<'a> Visitor<'a> for RoleRelationshipsVisitor {
            type Value = RoleRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut permissions: Option<crate::datadogV2::model::RelationshipToPermissions> =
                    None;
                let mut users: Option<crate::datadogV2::model::RelationshipToUsers> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "permissions" => {
                            if v.is_null() {
                                continue;
                            }
                            permissions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "users" => {
                            if v.is_null() {
                                continue;
                            }
                            users = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = RoleRelationships {
                    permissions,
                    users,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RoleRelationshipsVisitor)
    }
}
