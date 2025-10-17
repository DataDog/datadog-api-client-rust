// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Name of the target attributes which value is defined by the matching category.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsSchemaCategoryMapperTargets {
    /// ID of the field to map log attributes to.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the field to map log attributes to.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsSchemaCategoryMapperTargets {
    pub fn new() -> LogsSchemaCategoryMapperTargets {
        LogsSchemaCategoryMapperTargets {
            id: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for LogsSchemaCategoryMapperTargets {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsSchemaCategoryMapperTargets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsSchemaCategoryMapperTargetsVisitor;
        impl<'a> Visitor<'a> for LogsSchemaCategoryMapperTargetsVisitor {
            type Value = LogsSchemaCategoryMapperTargets;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LogsSchemaCategoryMapperTargets {
                    id,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsSchemaCategoryMapperTargetsVisitor)
    }
}
