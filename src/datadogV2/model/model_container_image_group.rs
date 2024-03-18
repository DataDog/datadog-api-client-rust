// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Container Image Group object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageGroup {
    /// Attributes for a Container Image Group.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ContainerImageGroupAttributes>,
    /// Container Image Group ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships inside a Container Image Group.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::ContainerImageGroupRelationships>,
    /// Type of Container Image Group.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerImageGroupType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageGroup {
    pub fn new() -> ContainerImageGroup {
        ContainerImageGroup {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::ContainerImageGroupAttributes,
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
        value: crate::datadogV2::model::ContainerImageGroupRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ContainerImageGroupType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ContainerImageGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageGroupVisitor;
        impl<'a> Visitor<'a> for ContainerImageGroupVisitor {
            type Value = ContainerImageGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::ContainerImageGroupAttributes> =
                    None;
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::ContainerImageGroupRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::ContainerImageGroupType> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ContainerImageGroupType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageGroup {
                    attributes,
                    id,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageGroupVisitor)
    }
}
