// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the role object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoleResponseRelationships {
    /// Relationship to multiple permissions objects.
    #[serde(rename = "permissions")]
    pub permissions: Option<crate::datadogV2::model::RelationshipToPermissions>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoleResponseRelationships {
    pub fn new() -> RoleResponseRelationships {
        RoleResponseRelationships {
            permissions: None,
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
}

impl Default for RoleResponseRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RoleResponseRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoleResponseRelationshipsVisitor;
        impl<'a> Visitor<'a> for RoleResponseRelationshipsVisitor {
            type Value = RoleResponseRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut permissions: Option<crate::datadogV2::model::RelationshipToPermissions> =
                    None;
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
                        &_ => {}
                    }
                }

                let content = RoleResponseRelationships {
                    permissions,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RoleResponseRelationshipsVisitor)
    }
}
