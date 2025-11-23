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
pub struct GetCohortResponseDataAttributesCohortsItemsValuesItems {
    #[serde(rename = "absolute_value")]
    pub absolute_value: Option<i64>,
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    #[serde(rename = "relative_value")]
    pub relative_value: Option<f64>,
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    #[serde(rename = "window")]
    pub window: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetCohortResponseDataAttributesCohortsItemsValuesItems {
    pub fn new() -> GetCohortResponseDataAttributesCohortsItemsValuesItems {
        GetCohortResponseDataAttributesCohortsItemsValuesItems {
            absolute_value: None,
            end_time: None,
            relative_value: None,
            start_time: None,
            window: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn absolute_value(mut self, value: i64) -> Self {
        self.absolute_value = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn relative_value(mut self, value: f64) -> Self {
        self.relative_value = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn window(mut self, value: i64) -> Self {
        self.window = Some(value);
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

impl Default for GetCohortResponseDataAttributesCohortsItemsValuesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetCohortResponseDataAttributesCohortsItemsValuesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetCohortResponseDataAttributesCohortsItemsValuesItemsVisitor;
        impl<'a> Visitor<'a> for GetCohortResponseDataAttributesCohortsItemsValuesItemsVisitor {
            type Value = GetCohortResponseDataAttributesCohortsItemsValuesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut absolute_value: Option<i64> = None;
                let mut end_time: Option<i64> = None;
                let mut relative_value: Option<f64> = None;
                let mut start_time: Option<i64> = None;
                let mut window: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "absolute_value" => {
                            if v.is_null() {
                                continue;
                            }
                            absolute_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_time" => {
                            if v.is_null() {
                                continue;
                            }
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relative_value" => {
                            if v.is_null() {
                                continue;
                            }
                            relative_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_time" => {
                            if v.is_null() {
                                continue;
                            }
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "window" => {
                            if v.is_null() {
                                continue;
                            }
                            window = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetCohortResponseDataAttributesCohortsItemsValuesItems {
                    absolute_value,
                    end_time,
                    relative_value,
                    start_time,
                    window,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetCohortResponseDataAttributesCohortsItemsValuesItemsVisitor)
    }
}
