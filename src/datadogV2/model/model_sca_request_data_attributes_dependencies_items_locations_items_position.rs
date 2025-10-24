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
pub struct ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
    #[serde(rename = "col")]
    pub col: Option<i32>,
    #[serde(rename = "line")]
    pub line: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
    pub fn new() -> ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
        ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
            col: None,
            line: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn col(mut self, value: i32) -> Self {
        self.col = Some(value);
        self
    }

    pub fn line(mut self, value: i32) -> Self {
        self.line = Some(value);
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

impl Default for ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesDependenciesItemsLocationsItemsPositionVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesDependenciesItemsLocationsItemsPositionVisitor {
            type Value = ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut col: Option<i32> = None;
                let mut line: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "col" => {
                            if v.is_null() {
                                continue;
                            }
                            col = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "line" => {
                            if v.is_null() {
                                continue;
                            }
                            line = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition {
                    col,
                    line,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ScaRequestDataAttributesDependenciesItemsLocationsItemsPositionVisitor)
    }
}
