// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for creating a role assignment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRoleAssignmentRelationshipsRequest {
    /// Relationship to a role.
    #[serde(rename = "reserved_role")]
    pub reserved_role: Option<crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship>,
    /// Relationship to a responder.
    #[serde(rename = "responder")]
    pub responder: crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
    /// Relationship to a role.
    #[serde(rename = "user_defined_role")]
    pub user_defined_role: Option<crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRoleAssignmentRelationshipsRequest {
    pub fn new(
        responder: crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
    ) -> IncidentRoleAssignmentRelationshipsRequest {
        IncidentRoleAssignmentRelationshipsRequest {
            reserved_role: None,
            responder,
            user_defined_role: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reserved_role(
        mut self,
        value: crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship,
    ) -> Self {
        self.reserved_role = Some(value);
        self
    }

    pub fn user_defined_role(
        mut self,
        value: crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship,
    ) -> Self {
        self.user_defined_role = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentRoleAssignmentRelationshipsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRoleAssignmentRelationshipsRequestVisitor;
        impl<'a> Visitor<'a> for IncidentRoleAssignmentRelationshipsRequestVisitor {
            type Value = IncidentRoleAssignmentRelationshipsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reserved_role: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship,
                > = None;
                let mut responder: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
                > = None;
                let mut user_defined_role: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentRoleRelationship,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reserved_role" => {
                            if v.is_null() {
                                continue;
                            }
                            reserved_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responder" => {
                            responder = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_defined_role" => {
                            if v.is_null() {
                                continue;
                            }
                            user_defined_role =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let responder = responder.ok_or_else(|| M::Error::missing_field("responder"))?;

                let content = IncidentRoleAssignmentRelationshipsRequest {
                    reserved_role,
                    responder,
                    user_defined_role,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRoleAssignmentRelationshipsRequestVisitor)
    }
}
