// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Axis {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "max")]
    pub max: Option<crate::datadogV1_20270101::model::AxisBound>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "min")]
    pub min: Option<crate::datadogV1_20270101::model::AxisBound>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scale")]
    pub scale: Option<crate::datadogV1_20270101::model::ScaleType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "units")]
    pub units: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            include_zero: None,
            label: None,
            max: None,
            min: None,
            scale: None,
            units: None,
            _unparsed: false,
        }
    }
    
    pub fn include_zero(mut self, value: bool) -> Self {
        self.include_zero = Some(value);
        self
    }
    
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    
    pub fn max(mut self, value: crate::datadogV1_20270101::model::AxisBound) -> Self {
        self.max = Some(value);
        self
    }
    
    pub fn min(mut self, value: crate::datadogV1_20270101::model::AxisBound) -> Self {
        self.min = Some(value);
        self
    }
    
    pub fn scale(mut self, value: crate::datadogV1_20270101::model::ScaleType) -> Self {
        self.scale = Some(value);
        self
    }
    
    pub fn units(mut self, value: bool) -> Self {
        self.units = Some(value);
        self
    }
    
}


impl Default for Axis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Axis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AxisVisitor;
        impl<'a> Visitor<'a> for AxisVisitor {
            type Value = Axis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_zero: Option<bool> = None;
                let mut label: Option<String> = None;
                let mut max: Option<crate::datadogV1_20270101::model::AxisBound> = None;
                let mut min: Option<crate::datadogV1_20270101::model::AxisBound> = None;
                let mut scale: Option<crate::datadogV1_20270101::model::ScaleType> = None;
                let mut units: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_zero" => {
                            if v.is_null() {
                                continue;
                            }
                            include_zero = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "max" => {
                            if v.is_null() {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _max) = max {
                                match _max {
                                    crate::datadogV1_20270101::model::AxisBound::UnparsedObject(_max) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "min" => {
                            if v.is_null() {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _min) = min {
                                match _min {
                                    crate::datadogV1_20270101::model::AxisBound::UnparsedObject(_min) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "scale" => {
                            if v.is_null() {
                                continue;
                            }
                            scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scale) = scale {
                                match _scale {
                                    crate::datadogV1_20270101::model::ScaleType::UnparsedObject(_scale) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "units" => {
                            if v.is_null() {
                                continue;
                            }
                            units = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = Axis {
                    include_zero,
                    label,
                    max,
                    min,
                    scale,
                    units,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AxisVisitor)
    }
}
