// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentHandleDataResponse {
    /// Incident handle attributes for responses
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentHandleAttributesResponse,
    /// The ID of the incident handle
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "relationships",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub relationships: Option<Option<crate::datadogV2::model::IncidentHandleRelationships>>,
    /// Incident handle resource type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentHandleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentHandleDataResponse {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentHandleAttributesResponse,
        id: String,
        type_: crate::datadogV2::model::IncidentHandleType,
    ) -> IncidentHandleDataResponse {
        IncidentHandleDataResponse {
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
        value: Option<crate::datadogV2::model::IncidentHandleRelationships>,
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

impl<'de> Deserialize<'de> for IncidentHandleDataResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentHandleDataResponseVisitor;
        impl<'a> Visitor<'a> for IncidentHandleDataResponseVisitor {
            type Value = IncidentHandleDataResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::IncidentHandleAttributesResponse,
                > = None;
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    Option<crate::datadogV2::model::IncidentHandleRelationships>,
                > = None;
                let mut type_: Option<crate::datadogV2::model::IncidentHandleType> = None;
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
                                    crate::datadogV2::model::IncidentHandleType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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

                let content = IncidentHandleDataResponse {
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

        deserializer.deserialize_any(IncidentHandleDataResponseVisitor)
    }
}
