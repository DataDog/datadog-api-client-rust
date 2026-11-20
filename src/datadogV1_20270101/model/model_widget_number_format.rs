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
pub struct WidgetNumberFormat {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "precision")]
    pub precision: Option<crate::datadogV1_20270101::model::Precision>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unit")]
    pub unit: Option<crate::datadogV1_20270101::model::NumberFormatUnit>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unit_scale", default, with = "::serde_with::rust::double_option")]
    pub unit_scale: Option<Option<crate::datadogV1_20270101::model::NumberFormatUnitScale>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl WidgetNumberFormat {
    pub fn new() -> WidgetNumberFormat {
        WidgetNumberFormat {
            precision: None,
            unit: None,
            unit_scale: None,
            _unparsed: false,
        }
    }
    
    pub fn precision(mut self, value: crate::datadogV1_20270101::model::Precision) -> Self {
        self.precision = Some(value);
        self
    }
    
    pub fn unit(mut self, value: crate::datadogV1_20270101::model::NumberFormatUnit) -> Self {
        self.unit = Some(value);
        self
    }
    
    pub fn unit_scale(mut self, value: Option<crate::datadogV1_20270101::model::NumberFormatUnitScale>) -> Self {
        self.unit_scale = Some(value);
        self
    }
    
}


impl Default for WidgetNumberFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetNumberFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetNumberFormatVisitor;
        impl<'a> Visitor<'a> for WidgetNumberFormatVisitor {
            type Value = WidgetNumberFormat;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut precision: Option<crate::datadogV1_20270101::model::Precision> = None;
                let mut unit: Option<crate::datadogV1_20270101::model::NumberFormatUnit> = None;
                let mut unit_scale: Option<Option<crate::datadogV1_20270101::model::NumberFormatUnitScale>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "precision" => {
                            if v.is_null() {
                                continue;
                            }
                            precision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _precision) = precision {
                                match _precision {
                                    crate::datadogV1_20270101::model::Precision::UnparsedObject(_precision) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1_20270101::model::NumberFormatUnit::UnparsedObject(_unit) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unit_scale" => {
                            unit_scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = WidgetNumberFormat {
                    precision,
                    unit,
                    unit_scale,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetNumberFormatVisitor)
    }
}
