// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's integration relationships from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentIntegrationRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentIntegrationRelationships {
    pub fn new() -> IncidentIntegrationRelationships {
        IncidentIntegrationRelationships {
            created_by_user: None,
            last_modified_by_user: None,
            _unparsed: false,
        }
    }

    pub fn created_by_user(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::RelationshipToUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }
}

impl Default for IncidentIntegrationRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentIntegrationRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentIntegrationRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentIntegrationRelationshipsVisitor {
            type Value = IncidentIntegrationRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut last_modified_by_user: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentIntegrationRelationships {
                    created_by_user,
                    last_modified_by_user,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentIntegrationRelationshipsVisitor)
    }
}
