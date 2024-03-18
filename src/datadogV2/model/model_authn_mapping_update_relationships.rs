// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationship of AuthN Mapping update object to Role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingUpdateRelationships {
    /// Relationship to role.
    #[serde(rename = "role")]
    pub role: Option<crate::datadogV2::model::RelationshipToRole>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingUpdateRelationships {
    pub fn new() -> AuthNMappingUpdateRelationships {
        AuthNMappingUpdateRelationships {
            role: None,
            _unparsed: false,
        }
    }

    pub fn role(mut self, value: crate::datadogV2::model::RelationshipToRole) -> Self {
        self.role = Some(value);
        self
    }
}

impl Default for AuthNMappingUpdateRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AuthNMappingUpdateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingUpdateRelationshipsVisitor;
        impl<'a> Visitor<'a> for AuthNMappingUpdateRelationshipsVisitor {
            type Value = AuthNMappingUpdateRelationships;

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
                            if v.is_null() {
                                continue;
                            }
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AuthNMappingUpdateRelationships { role, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingUpdateRelationshipsVisitor)
    }
}
