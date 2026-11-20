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
pub struct TimeseriesRequestStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "color_order")]
    pub color_order: Option<crate::datadogV1_20270101::model::TimeseriesColorOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_value_labels")]
    pub has_value_labels: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "line_type")]
    pub line_type: Option<crate::datadogV1_20270101::model::WidgetLineType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "line_width")]
    pub line_width: Option<crate::datadogV1_20270101::model::WidgetLineWidth>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1_20270101::model::WidgetStyleOrderBy>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order_reverse")]
    pub order_reverse: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value_labels_density")]
    pub value_labels_density: Option<crate::datadogV1_20270101::model::ValueLabelsDensity>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesRequestStyle {
    pub fn new() -> TimeseriesRequestStyle {
        TimeseriesRequestStyle {
            color_order: None,
            has_value_labels: None,
            line_type: None,
            line_width: None,
            order_by: None,
            order_reverse: None,
            palette: None,
            value_labels_density: None,
            _unparsed: false,
        }
    }
    
    pub fn color_order(mut self, value: crate::datadogV1_20270101::model::TimeseriesColorOrder) -> Self {
        self.color_order = Some(value);
        self
    }
    
    pub fn has_value_labels(mut self, value: bool) -> Self {
        self.has_value_labels = Some(value);
        self
    }
    
    pub fn line_type(mut self, value: crate::datadogV1_20270101::model::WidgetLineType) -> Self {
        self.line_type = Some(value);
        self
    }
    
    pub fn line_width(mut self, value: crate::datadogV1_20270101::model::WidgetLineWidth) -> Self {
        self.line_width = Some(value);
        self
    }
    
    pub fn order_by(mut self, value: crate::datadogV1_20270101::model::WidgetStyleOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }
    
    pub fn order_reverse(mut self, value: bool) -> Self {
        self.order_reverse = Some(value);
        self
    }
    
    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
    
    pub fn value_labels_density(mut self, value: crate::datadogV1_20270101::model::ValueLabelsDensity) -> Self {
        self.value_labels_density = Some(value);
        self
    }
    
}


impl Default for TimeseriesRequestStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimeseriesRequestStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesRequestStyleVisitor;
        impl<'a> Visitor<'a> for TimeseriesRequestStyleVisitor {
            type Value = TimeseriesRequestStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_order: Option<crate::datadogV1_20270101::model::TimeseriesColorOrder> = None;
                let mut has_value_labels: Option<bool> = None;
                let mut line_type: Option<crate::datadogV1_20270101::model::WidgetLineType> = None;
                let mut line_width: Option<crate::datadogV1_20270101::model::WidgetLineWidth> = None;
                let mut order_by: Option<crate::datadogV1_20270101::model::WidgetStyleOrderBy> = None;
                let mut order_reverse: Option<bool> = None;
                let mut palette: Option<String> = None;
                let mut value_labels_density: Option<crate::datadogV1_20270101::model::ValueLabelsDensity> = None;
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
                        "has_value_labels" => {
                            if v.is_null() {
                                continue;
                            }
                            has_value_labels = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "line_type" => {
                            if v.is_null() {
                                continue;
                            }
                            line_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_type) = line_type {
                                match _line_type {
                                    crate::datadogV1_20270101::model::WidgetLineType::UnparsedObject(_line_type) => {
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
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_by) = order_by {
                                match _order_by {
                                    crate::datadogV1_20270101::model::WidgetStyleOrderBy::UnparsedObject(_order_by) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "order_reverse" => {
                            if v.is_null() {
                                continue;
                            }
                            order_reverse = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value_labels_density" => {
                            if v.is_null() {
                                continue;
                            }
                            value_labels_density = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value_labels_density) = value_labels_density {
                                match _value_labels_density {
                                    crate::datadogV1_20270101::model::ValueLabelsDensity::UnparsedObject(_value_labels_density) => {
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

                let content = TimeseriesRequestStyle {
                    color_order,
                    has_value_labels,
                    line_type,
                    line_width,
                    order_by,
                    order_reverse,
                    palette,
                    value_labels_density,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesRequestStyleVisitor)
    }
}
