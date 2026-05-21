// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a role assignment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRoleAssignmentRelationshipsResponse {
    /// Relationship to a responder.
    #[serde(rename = "created_by_user")]
    pub created_by_user:
        Option<crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship>,
    /// Relationship to a responder.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user:
        Option<crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship>,
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

impl IncidentRoleAssignmentRelationshipsResponse {
    pub fn new(
        responder: crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
    ) -> IncidentRoleAssignmentRelationshipsResponse {
        IncidentRoleAssignmentRelationshipsResponse {
            created_by_user: None,
            last_modified_by_user: None,
            reserved_role: None,
            responder,
            user_defined_role: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_user(
        mut self,
        value: crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
    ) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
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

impl<'de> Deserialize<'de> for IncidentRoleAssignmentRelationshipsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRoleAssignmentRelationshipsResponseVisitor;
        impl<'a> Visitor<'a> for IncidentRoleAssignmentRelationshipsResponseVisitor {
            type Value = IncidentRoleAssignmentRelationshipsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
                > = None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentResponderRelationship,
                > = None;
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

                let content = IncidentRoleAssignmentRelationshipsResponse {
                    created_by_user,
                    last_modified_by_user,
                    reserved_role,
                    responder,
                    user_defined_role,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRoleAssignmentRelationshipsResponseVisitor)
    }
}
