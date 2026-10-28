// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Role assignment data in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRoleAssignmentDataResponse {
    /// Attributes of an incident role assignment.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentRoleAssignmentDataAttributesResponse,
    /// The role assignment identifier.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Relationships of a role assignment.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentRoleAssignmentRelationshipsResponse>,
    /// Incident role assignment resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentRoleAssignmentType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRoleAssignmentDataResponse {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentRoleAssignmentDataAttributesResponse,
        id: uuid::Uuid,
        type_: crate::datadogV2::model::IncidentRoleAssignmentType,
    ) -> IncidentRoleAssignmentDataResponse {
        IncidentRoleAssignmentDataResponse {
            attributes,
            id,
            relationships: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::IncidentRoleAssignmentRelationshipsResponse,
    ) -> Self {
        self.relationships = Some(value);
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

impl<'de> Deserialize<'de> for IncidentRoleAssignmentDataResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRoleAssignmentDataResponseVisitor;
        impl<'a> Visitor<'a> for IncidentRoleAssignmentDataResponseVisitor {
            type Value = IncidentRoleAssignmentDataResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentDataAttributesResponse,
                > = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::IncidentRoleAssignmentRelationshipsResponse,
                > = None;
                let mut type_: Option<crate::datadogV2::model::IncidentRoleAssignmentType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            if v.is_null() {
                                continue;
                            }
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::IncidentRoleAssignmentType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = IncidentRoleAssignmentDataResponse {
                    attributes,
                    id,
                    relationships,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRoleAssignmentDataResponseVisitor)
    }
}
