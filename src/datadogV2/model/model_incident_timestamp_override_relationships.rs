// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for incident timestamp override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimestampOverrideRelationships {
    /// Relationship to a user.
    #[serde(rename = "created_by_user")]
    pub created_by_user: crate::datadogV2::model::TimestampOverrideUserRelationship,
    /// Relationship to a user.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user: crate::datadogV2::model::TimestampOverrideUserRelationship,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimestampOverrideRelationships {
    pub fn new(
        created_by_user: crate::datadogV2::model::TimestampOverrideUserRelationship,
        last_modified_by_user: crate::datadogV2::model::TimestampOverrideUserRelationship,
    ) -> IncidentTimestampOverrideRelationships {
        IncidentTimestampOverrideRelationships {
            created_by_user,
            last_modified_by_user,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentTimestampOverrideRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimestampOverrideRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentTimestampOverrideRelationshipsVisitor {
            type Value = IncidentTimestampOverrideRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<
                    crate::datadogV2::model::TimestampOverrideUserRelationship,
                > = None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::TimestampOverrideUserRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_user" => {
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_by_user =
                    created_by_user.ok_or_else(|| M::Error::missing_field("created_by_user"))?;
                let last_modified_by_user = last_modified_by_user
                    .ok_or_else(|| M::Error::missing_field("last_modified_by_user"))?;

                let content = IncidentTimestampOverrideRelationships {
                    created_by_user,
                    last_modified_by_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimestampOverrideRelationshipsVisitor)
    }
}
