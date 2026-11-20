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
pub struct BackendTimeseriesAxisDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scale")]
    pub scale: Option<crate::datadogV1_20270101::model::ScaleType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_unit")]
    pub show_unit: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "stack")]
    pub stack: Option<crate::datadogV1_20270101::model::TimeseriesStackMode>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl BackendTimeseriesAxisDefinition {
    pub fn new() -> BackendTimeseriesAxisDefinition {
        BackendTimeseriesAxisDefinition {
            include_zero: None,
            label: None,
            max: None,
            min: None,
            scale: None,
            show_unit: None,
            stack: None,
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
    
    pub fn max(mut self, value: String) -> Self {
        self.max = Some(value);
        self
    }
    
    pub fn min(mut self, value: String) -> Self {
        self.min = Some(value);
        self
    }
    
    pub fn scale(mut self, value: crate::datadogV1_20270101::model::ScaleType) -> Self {
        self.scale = Some(value);
        self
    }
    
    pub fn show_unit(mut self, value: bool) -> Self {
        self.show_unit = Some(value);
        self
    }
    
    pub fn stack(mut self, value: crate::datadogV1_20270101::model::TimeseriesStackMode) -> Self {
        self.stack = Some(value);
        self
    }
    
}


impl Default for BackendTimeseriesAxisDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BackendTimeseriesAxisDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BackendTimeseriesAxisDefinitionVisitor;
        impl<'a> Visitor<'a> for BackendTimeseriesAxisDefinitionVisitor {
            type Value = BackendTimeseriesAxisDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_zero: Option<bool> = None;
                let mut label: Option<String> = None;
                let mut max: Option<String> = None;
                let mut min: Option<String> = None;
                let mut scale: Option<crate::datadogV1_20270101::model::ScaleType> = None;
                let mut show_unit: Option<bool> = None;
                let mut stack: Option<crate::datadogV1_20270101::model::TimeseriesStackMode> = None;
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
                        },
                        "min" => {
                            if v.is_null() {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "show_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            show_unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "stack" => {
                            if v.is_null() {
                                continue;
                            }
                            stack = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stack) = stack {
                                match _stack {
                                    crate::datadogV1_20270101::model::TimeseriesStackMode::UnparsedObject(_stack) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = BackendTimeseriesAxisDefinition {
                    include_zero,
                    label,
                    max,
                    min,
                    scale,
                    show_unit,
                    stack,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BackendTimeseriesAxisDefinitionVisitor)
    }
}
