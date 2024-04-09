// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship of AuthN Mapping to a Role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingRelationshipToRole {
    /// Relationship to role.
    #[serde(rename = "role")]
    pub role: crate::datadogV2::model::RelationshipToRole,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingRelationshipToRole {
    pub fn new(
        role: crate::datadogV2::model::RelationshipToRole,
    ) -> AuthNMappingRelationshipToRole {
        AuthNMappingRelationshipToRole {
            role,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AuthNMappingRelationshipToRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingRelationshipToRoleVisitor;
        impl<'a> Visitor<'a> for AuthNMappingRelationshipToRoleVisitor {
            type Value = AuthNMappingRelationshipToRole;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut role: Option<crate::datadogV2::model::RelationshipToRole> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;

                let content = AuthNMappingRelationshipToRole { role, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingRelationshipToRoleVisitor)
    }
}
