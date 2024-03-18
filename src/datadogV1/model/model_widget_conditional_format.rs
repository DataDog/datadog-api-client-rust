// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define a conditional format for the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetConditionalFormat {
    /// Comparator to apply.
    #[serde(rename = "comparator")]
    pub comparator: crate::datadogV1::model::WidgetComparator,
    /// Color palette to apply to the background, same values available as palette.
    #[serde(rename = "custom_bg_color")]
    pub custom_bg_color: Option<String>,
    /// Color palette to apply to the foreground, same values available as palette.
    #[serde(rename = "custom_fg_color")]
    pub custom_fg_color: Option<String>,
    /// True hides values.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Displays an image as the background.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Metric from the request to correlate this conditional format with.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Color palette to apply.
    #[serde(rename = "palette")]
    pub palette: crate::datadogV1::model::WidgetPalette,
    /// Defines the displayed timeframe.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<String>,
    /// Value for the comparator.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetConditionalFormat {
    pub fn new(
        comparator: crate::datadogV1::model::WidgetComparator,
        palette: crate::datadogV1::model::WidgetPalette,
        value: f64,
    ) -> WidgetConditionalFormat {
        WidgetConditionalFormat {
            comparator,
            custom_bg_color: None,
            custom_fg_color: None,
            hide_value: None,
            image_url: None,
            metric: None,
            palette,
            timeframe: None,
            value,
            _unparsed: false,
        }
    }

    pub fn custom_bg_color(mut self, value: String) -> Self {
        self.custom_bg_color = Some(value);
        self
    }

    pub fn custom_fg_color(mut self, value: String) -> Self {
        self.custom_fg_color = Some(value);
        self
    }

    pub fn hide_value(mut self, value: bool) -> Self {
        self.hide_value = Some(value);
        self
    }

    pub fn image_url(mut self, value: String) -> Self {
        self.image_url = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn timeframe(mut self, value: String) -> Self {
        self.timeframe = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for WidgetConditionalFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetConditionalFormatVisitor;
        impl<'a> Visitor<'a> for WidgetConditionalFormatVisitor {
            type Value = WidgetConditionalFormat;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comparator: Option<crate::datadogV1::model::WidgetComparator> = None;
                let mut custom_bg_color: Option<String> = None;
                let mut custom_fg_color: Option<String> = None;
                let mut hide_value: Option<bool> = None;
                let mut image_url: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut palette: Option<crate::datadogV1::model::WidgetPalette> = None;
                let mut timeframe: Option<String> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comparator" => {
                            comparator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _comparator) = comparator {
                                match _comparator {
                                    crate::datadogV1::model::WidgetComparator::UnparsedObject(
                                        _comparator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "custom_bg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_bg_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_fg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_fg_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hide_value" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "image_url" => {
                            if v.is_null() {
                                continue;
                            }
                            image_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "palette" => {
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _palette) = palette {
                                match _palette {
                                    crate::datadogV1::model::WidgetPalette::UnparsedObject(
                                        _palette,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let comparator = comparator.ok_or_else(|| M::Error::missing_field("comparator"))?;
                let palette = palette.ok_or_else(|| M::Error::missing_field("palette"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = WidgetConditionalFormat {
                    comparator,
                    custom_bg_color,
                    custom_fg_color,
                    hide_value,
                    image_url,
                    metric,
                    palette,
                    timeframe,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetConditionalFormatVisitor)
    }
}
