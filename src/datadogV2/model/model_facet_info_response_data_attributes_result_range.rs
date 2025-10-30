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
pub struct FacetInfoResponseDataAttributesResultRange {
    #[serde(rename = "max")]
    pub max: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "min")]
    pub min: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FacetInfoResponseDataAttributesResultRange {
    pub fn new() -> FacetInfoResponseDataAttributesResultRange {
        FacetInfoResponseDataAttributesResultRange {
            max: None,
            min: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn max(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.min = Some(value);
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

impl Default for FacetInfoResponseDataAttributesResultRange {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FacetInfoResponseDataAttributesResultRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FacetInfoResponseDataAttributesResultRangeVisitor;
        impl<'a> Visitor<'a> for FacetInfoResponseDataAttributesResultRangeVisitor {
            type Value = FacetInfoResponseDataAttributesResultRange;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut min: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max" => {
                            if v.is_null() {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min" => {
                            if v.is_null() {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FacetInfoResponseDataAttributesResultRange {
                    max,
                    min,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FacetInfoResponseDataAttributesResultRangeVisitor)
    }
}
