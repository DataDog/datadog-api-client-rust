// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a container group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerGroupAttributes {
    /// Number of containers in the group.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Tags from the group name parsed in key/value format.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerGroupAttributes {
    pub fn new() -> ContainerGroupAttributes {
        ContainerGroupAttributes {
            count: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for ContainerGroupAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerGroupAttributesVisitor;
        impl<'a> Visitor<'a> for ContainerGroupAttributesVisitor {
            type Value = ContainerGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut tags: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerGroupAttributes {
                    count,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerGroupAttributesVisitor)
    }
}
