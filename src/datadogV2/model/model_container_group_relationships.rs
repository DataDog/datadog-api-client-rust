// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships to containers inside a container group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerGroupRelationships {
    /// Relationships to Containers inside a Container Group.
    #[serde(rename = "containers")]
    pub containers: Option<crate::datadogV2::model::ContainerGroupRelationshipsLink>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerGroupRelationships {
    pub fn new() -> ContainerGroupRelationships {
        ContainerGroupRelationships {
            containers: None,
            _unparsed: false,
        }
    }

    pub fn containers(
        &mut self,
        value: crate::datadogV2::model::ContainerGroupRelationshipsLink,
    ) -> &mut Self {
        self.containers = Some(value);
        self
    }
}

impl Default for ContainerGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerGroupRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerGroupRelationshipsVisitor;
        impl<'a> Visitor<'a> for ContainerGroupRelationshipsVisitor {
            type Value = ContainerGroupRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut containers: Option<
                    crate::datadogV2::model::ContainerGroupRelationshipsLink,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "containers" => {
                            if v.is_null() {
                                continue;
                            }
                            containers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerGroupRelationships {
                    containers,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerGroupRelationshipsVisitor)
    }
}
