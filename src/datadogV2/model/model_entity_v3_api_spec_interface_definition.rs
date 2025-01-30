// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `EntityV3APISpecInterfaceDefinition` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3APISpecInterfaceDefinition {
    /// The API definition.
    #[serde(rename = "definition")]
    pub definition: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3APISpecInterfaceDefinition {
    pub fn new() -> EntityV3APISpecInterfaceDefinition {
        EntityV3APISpecInterfaceDefinition {
            definition: None,
            _unparsed: false,
        }
    }

    pub fn definition(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.definition = Some(value);
        self
    }
}

impl Default for EntityV3APISpecInterfaceDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3APISpecInterfaceDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3APISpecInterfaceDefinitionVisitor;
        impl<'a> Visitor<'a> for EntityV3APISpecInterfaceDefinitionVisitor {
            type Value = EntityV3APISpecInterfaceDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            if v.is_null() {
                                continue;
                            }
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3APISpecInterfaceDefinition {
                    definition,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3APISpecInterfaceDefinitionVisitor)
    }
}
