// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of Entity V3 System Spec object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3SystemSpec {
    /// A list of components belongs to the system.
    #[serde(rename = "components")]
    pub components: Option<Vec<String>>,
    /// The lifecycle state of the component.
    #[serde(rename = "lifecycle")]
    pub lifecycle: Option<String>,
    /// An entity reference to the owner of the component.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3SystemSpec {
    pub fn new() -> EntityV3SystemSpec {
        EntityV3SystemSpec {
            components: None,
            lifecycle: None,
            tier: None,
            _unparsed: false,
        }
    }

    pub fn components(mut self, value: Vec<String>) -> Self {
        self.components = Some(value);
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
}

impl Default for EntityV3SystemSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3SystemSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3SystemSpecVisitor;
        impl<'a> Visitor<'a> for EntityV3SystemSpecVisitor {
            type Value = EntityV3SystemSpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components: Option<Vec<String>> = None;
                let mut lifecycle: Option<String> = None;
                let mut tier: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3SystemSpec {
                    components,
                    lifecycle,
                    tier,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3SystemSpecVisitor)
    }
}
