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
pub struct RunChartLineStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "color_order")]
    pub color_order: Option<crate::datadogV1_20270101::model::TimeseriesColorOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "line_width")]
    pub line_width: Option<crate::datadogV1_20270101::model::WidgetLineWidth>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RunChartLineStyle {
    pub fn new() -> RunChartLineStyle {
        RunChartLineStyle {
            color_order: None,
            line_width: None,
            palette: None,
            _unparsed: false,
        }
    }
    
    pub fn color_order(mut self, value: crate::datadogV1_20270101::model::TimeseriesColorOrder) -> Self {
        self.color_order = Some(value);
        self
    }
    
    pub fn line_width(mut self, value: crate::datadogV1_20270101::model::WidgetLineWidth) -> Self {
        self.line_width = Some(value);
        self
    }
    
    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
    
}


impl Default for RunChartLineStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RunChartLineStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunChartLineStyleVisitor;
        impl<'a> Visitor<'a> for RunChartLineStyleVisitor {
            type Value = RunChartLineStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_order: Option<crate::datadogV1_20270101::model::TimeseriesColorOrder> = None;
                let mut line_width: Option<crate::datadogV1_20270101::model::WidgetLineWidth> = None;
                let mut palette: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_order" => {
                            if v.is_null() {
                                continue;
                            }
                            color_order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color_order) = color_order {
                                match _color_order {
                                    crate::datadogV1_20270101::model::TimeseriesColorOrder::UnparsedObject(_color_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "line_width" => {
                            if v.is_null() {
                                continue;
                            }
                            line_width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_width) = line_width {
                                match _line_width {
                                    crate::datadogV1_20270101::model::WidgetLineWidth::UnparsedObject(_line_width) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = RunChartLineStyle {
                    color_order,
                    line_width,
                    palette,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RunChartLineStyleVisitor)
    }
}
