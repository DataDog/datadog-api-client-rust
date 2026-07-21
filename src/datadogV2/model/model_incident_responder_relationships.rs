// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships for an incident responder.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentResponderRelationships {
    /// Relationship to user.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to user.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: Option<crate::datadogV2::model::RelationshipToUser>,
    /// Relationship to role assignments for a responder.
    #[serde(rename = "role_assignments")]
    pub role_assignments:
        Option<crate::datadogV2::model::IncidentResponderRoleAssignmentsRelationship>,
    /// Relationship to user.
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option")]
    pub user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentResponderRelationships {
    pub fn new() -> IncidentResponderRelationships {
        IncidentResponderRelationships {
            created_by: None,
            last_modified_by: None,
            role_assignments: None,
            user: None,
            additional_properties: std::collections::BTreeMap::new(),
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

    pub fn role_assignments(
        mut self,
        value: crate::datadogV2::model::IncidentResponderRoleAssignmentsRelationship,
    ) -> Self {
        self.role_assignments = Some(value);
        self
    }

    pub fn user(
        mut self,
        value: Option<crate::datadogV2::model::NullableRelationshipToUser>,
    ) -> Self {
        self.user = Some(value);
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

impl Default for IncidentResponderRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentResponderRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentResponderRelationshipsVisitor;
        impl<'a> Visitor<'a> for IncidentResponderRelationshipsVisitor {
            type Value = IncidentResponderRelationships;

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
                let mut role_assignments: Option<
                    crate::datadogV2::model::IncidentResponderRoleAssignmentsRelationship,
                > = None;
                let mut user: Option<Option<crate::datadogV2::model::NullableRelationshipToUser>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "role_assignments" => {
                            if v.is_null() {
                                continue;
                            }
                            role_assignments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user" => {
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentResponderRelationships {
                    created_by,
                    last_modified_by,
                    role_assignments,
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentResponderRelationshipsVisitor)
    }
}
