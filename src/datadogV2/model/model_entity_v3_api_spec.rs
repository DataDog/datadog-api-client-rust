// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 API Spec object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3APISpec {
    /// Services which implemented the API.
    #[serde(rename = "implementedBy")]
    pub implemented_by: Option<Vec<String>>,
    /// The API definition.
    #[serde(rename = "interface")]
    pub interface: Option<crate::datadogV2::model::EntityV3APISpecInterface>,
    /// The lifecycle state of the component.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// The importance of the component.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    /// The type of API.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3APISpec {
    pub fn new() -> EntityV3APISpec {
        EntityV3APISpec {
            implemented_by: None,
            interface: None,
            lifecycle: None,
            tier: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn implemented_by(mut self, value: Vec<String>) -> Self {
        self.implemented_by = Some(value);
        self
    }

    pub fn interface(mut self, value: crate::datadogV2::model::EntityV3APISpecInterface) -> Self {
        self.interface = Some(value);
        self
    }

    pub fn lifecycle(mut self, value: String) -> Self {
        self.lifecycle = Some(value);
        self
    }

    pub fn tier(mut self, value: String) -> Self {
        self.tier = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for EntityV3APISpec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3APISpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3APISpecVisitor;
        impl<'a> Visitor<'a> for EntityV3APISpecVisitor {
            type Value = EntityV3APISpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut implemented_by: Option<Vec<String>> = None;
                let mut interface: Option<crate::datadogV2::model::EntityV3APISpecInterface> = None;
                let mut lifecycle: Option<String> = None;
                let mut tier: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "implementedBy" => {
                            if v.is_null() {
                                continue;
                            }
                            implemented_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interface" => {
                            if v.is_null() {
                                continue;
                            }
                            interface = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _interface) = interface {
                                match _interface {
                                    crate::datadogV2::model::EntityV3APISpecInterface::UnparsedObject(_interface) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "lifecycle" => {
                            if v.is_null() {
                                continue;
                            }
                            lifecycle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tier" => {
                            if v.is_null() {
                                continue;
                            }
                            tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3APISpec {
                    implemented_by,
                    interface,
                    lifecycle,
                    tier,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3APISpecVisitor)
    }
}
