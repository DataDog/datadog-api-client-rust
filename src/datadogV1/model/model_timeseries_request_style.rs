// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define request widget style for timeseries widgets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesRequestStyle {
    /// If true, the value is displayed as a label relative to the data point.
    #[serde(rename = "has_value_labels")]
    pub has_value_labels: Option<bool>,
    /// Type of lines displayed.
    #[serde(rename = "line_type")]
    pub line_type: Option<crate::datadogV1::model::WidgetLineType>,
    /// Width of line displayed.
    #[serde(rename = "line_width")]
    pub line_width: Option<crate::datadogV1::model::WidgetLineWidth>,
    /// How to order series in timeseries visualizations.
    /// - `tags`: Order series alphabetically by tag name (default behavior)
    /// - `values`: Order series by their current metric values (typically descending)
    #[serde(rename = "order_by")]
    pub order_by: Option<crate::datadogV1::model::WidgetStyleOrderBy>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesRequestStyle {
    pub fn new() -> TimeseriesRequestStyle {
        TimeseriesRequestStyle {
            has_value_labels: None,
            line_type: None,
            line_width: None,
            order_by: None,
            palette: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn has_value_labels(mut self, value: bool) -> Self {
        self.has_value_labels = Some(value);
        self
    }

    pub fn line_type(mut self, value: crate::datadogV1::model::WidgetLineType) -> Self {
        self.line_type = Some(value);
        self
    }

    pub fn line_width(mut self, value: crate::datadogV1::model::WidgetLineWidth) -> Self {
        self.line_width = Some(value);
        self
    }

    pub fn order_by(mut self, value: crate::datadogV1::model::WidgetStyleOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
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
                let mut has_value_labels: Option<bool> = None;
                let mut line_type: Option<crate::datadogV1::model::WidgetLineType> = None;
                let mut line_width: Option<crate::datadogV1::model::WidgetLineWidth> = None;
                let mut order_by: Option<crate::datadogV1::model::WidgetStyleOrderBy> = None;
                let mut palette: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_value_labels" => {
                            if v.is_null() {
                                continue;
                            }
                            has_value_labels =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "line_type" => {
                            if v.is_null() {
                                continue;
                            }
                            line_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_type) = line_type {
                                match _line_type {
                                    crate::datadogV1::model::WidgetLineType::UnparsedObject(
                                        _line_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "line_width" => {
                            if v.is_null() {
                                continue;
                            }
                            line_width = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _line_width) = line_width {
                                match _line_width {
                                    crate::datadogV1::model::WidgetLineWidth::UnparsedObject(
                                        _line_width,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "order_by" => {
                            if v.is_null() {
                                continue;
                            }
                            order_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order_by) = order_by {
                                match _order_by {
                                    crate::datadogV1::model::WidgetStyleOrderBy::UnparsedObject(
                                        _order_by,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TimeseriesRequestStyle {
                    has_value_labels,
                    line_type,
                    line_width,
                    order_by,
                    palette,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesRequestStyleVisitor)
    }
}
