// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Role object returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoleUpdateResponseData {
    /// Attributes of the role.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::RoleUpdateAttributes>,
    /// The unique identifier of the role.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the role object returned by the API.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::RoleResponseRelationships>,
    /// Roles type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RolesType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoleUpdateResponseData {
    pub fn new(type_: crate::datadogV2::model::RolesType) -> RoleUpdateResponseData {
        RoleUpdateResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::RoleUpdateAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::RoleResponseRelationships,
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

impl<'de> Deserialize<'de> for RoleUpdateResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoleUpdateResponseDataVisitor;
        impl<'a> Visitor<'a> for RoleUpdateResponseDataVisitor {
            type Value = RoleUpdateResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::RoleUpdateAttributes> = None;
                let mut id: Option<String> = None;
                let mut relationships: Option<crate::datadogV2::model::RoleResponseRelationships> =
                    None;
                let mut type_: Option<crate::datadogV2::model::RolesType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV2::model::RolesType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = RoleUpdateResponseData {
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

        deserializer.deserialize_any(RoleUpdateResponseDataVisitor)
    }
}
