// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident Team data for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTeamUpdateData {
    /// The incident team's attributes for an update request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentTeamUpdateAttributes>,
    /// The incident team's ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The incident team's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentTeamRelationships>,
    /// Incident Team resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentTeamType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTeamUpdateData {
    pub fn new(type_: crate::datadogV2::model::IncidentTeamType) -> IncidentTeamUpdateData {
        IncidentTeamUpdateData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::IncidentTeamUpdateAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::IncidentTeamRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentTeamUpdateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTeamUpdateDataVisitor;
        impl<'a> Visitor<'a> for IncidentTeamUpdateDataVisitor {
            type Value = IncidentTeamUpdateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::IncidentTeamUpdateAttributes> =
                    None;
                let mut id: Option<String> = None;
                let mut relationships: Option<crate::datadogV2::model::IncidentTeamRelationships> =
                    None;
                let mut type_: Option<crate::datadogV2::model::IncidentTeamType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
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
                                    crate::datadogV2::model::IncidentTeamType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = IncidentTeamUpdateData {
                    attributes,
                    id,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTeamUpdateDataVisitor)
    }
}
