// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data for creating an AuthN Mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthNMappingCreateData {
    /// Key/Value pair of attributes used for create request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::AuthNMappingCreateAttributes>,
    /// Relationship of AuthN Mapping create object to Role.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::AuthNMappingCreateRelationships>,
    /// AuthN Mappings resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AuthNMappingsType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthNMappingCreateData {
    pub fn new(type_: crate::datadogV2::model::AuthNMappingsType) -> AuthNMappingCreateData {
        AuthNMappingCreateData {
            attributes: None,
            relationships: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingCreateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::AuthNMappingCreateRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AuthNMappingCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthNMappingCreateDataVisitor;
        impl<'a> Visitor<'a> for AuthNMappingCreateDataVisitor {
            type Value = AuthNMappingCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::AuthNMappingCreateAttributes> =
                    None;
                let mut relationships: Option<
                    crate::datadogV2::model::AuthNMappingCreateRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::AuthNMappingsType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::AuthNMappingsType::UnparsedObject(
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

                let content = AuthNMappingCreateData {
                    attributes,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthNMappingCreateDataVisitor)
    }
}
