// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident role assignment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRoleAssignmentDataAttributesResponse {
    /// Timestamp when the role assignment was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the role assignment was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// The type of the responder.
    #[serde(rename = "responder_type")]
    pub responder_type: String,
    /// The name of the assigned role.
    #[serde(rename = "role")]
    pub role: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRoleAssignmentDataAttributesResponse {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        modified: chrono::DateTime<chrono::Utc>,
        responder_type: String,
        role: String,
    ) -> IncidentRoleAssignmentDataAttributesResponse {
        IncidentRoleAssignmentDataAttributesResponse {
            created,
            modified,
            responder_type,
            role,
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

impl<'de> Deserialize<'de> for IncidentRoleAssignmentDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRoleAssignmentDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentRoleAssignmentDataAttributesResponseVisitor {
            type Value = IncidentRoleAssignmentDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut responder_type: Option<String> = None;
                let mut role: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responder_type" => {
                            responder_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role" => {
                            role = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let responder_type =
                    responder_type.ok_or_else(|| M::Error::missing_field("responder_type"))?;
                let role = role.ok_or_else(|| M::Error::missing_field("role"))?;

                let content = IncidentRoleAssignmentDataAttributesResponse {
                    created,
                    modified,
                    responder_type,
                    role,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRoleAssignmentDataAttributesResponseVisitor)
    }
}
