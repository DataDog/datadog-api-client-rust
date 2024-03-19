// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident service's relationships.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentServiceRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: Option<crate::datadogV2::model::RelationshipToUser>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentServiceRelationships {
    pub fn new() -> IncidentServiceRelationships {
        IncidentServiceRelationships {
            created_by: None,
            last_modified_by: None,
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn last_modified_by(mut self, value: crate::datadogV2::model::RelationshipToUser) -> Self {
        self.last_modified_by = Some(value);
        self
    }
}

impl Default for IncidentServiceRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentServiceRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentServiceRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentServiceRelationshipsVisitor {
            type Value = IncidentServiceRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<crate::datadogV2::model::RelationshipToUser> = None;
                let mut last_modified_by: Option<crate::datadogV2::model::RelationshipToUser> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentServiceRelationships {
                    created_by,
                    last_modified_by,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentServiceRelationshipsVisitor)
    }
}
