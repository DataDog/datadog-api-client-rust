// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A shared dashboard response resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SharedDashboardResponse {
    /// Attributes of a shared dashboard response.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SharedDashboardResponseAttributes,
    /// ID of the shared dashboard.
    #[serde(rename = "id")]
    pub id: String,
    /// Relationships of a shared dashboard.
    #[serde(rename = "relationships")]
    pub relationships: crate::datadogV2::model::SharedDashboardRelationships,
    /// Shared dashboard resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SharedDashboardType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SharedDashboardResponse {
    pub fn new(
        attributes: crate::datadogV2::model::SharedDashboardResponseAttributes,
        id: String,
        relationships: crate::datadogV2::model::SharedDashboardRelationships,
        type_: crate::datadogV2::model::SharedDashboardType,
    ) -> SharedDashboardResponse {
        SharedDashboardResponse {
            attributes,
            id,
            relationships,
            type_,
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

impl<'de> Deserialize<'de> for SharedDashboardResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SharedDashboardResponseVisitor;
        impl<'a> Visitor<'a> for SharedDashboardResponseVisitor {
            type Value = SharedDashboardResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SharedDashboardResponseAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::SharedDashboardRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::SharedDashboardType> = None;
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
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SharedDashboardType::UnparsedObject(_type_) => {
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
                let relationships =
                    relationships.ok_or_else(|| M::Error::missing_field("relationships"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SharedDashboardResponse {
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

        deserializer.deserialize_any(SharedDashboardResponseVisitor)
    }
}
