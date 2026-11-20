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
pub struct ThresholdFormattingRule {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "comparator")]
    pub comparator: crate::datadogV1_20270101::model::Comparator,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_bg_color")]
    pub custom_bg_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_fg_color")]
    pub custom_fg_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "highlight_row")]
    pub highlight_row: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ThresholdFormattingRule {
    pub fn new(comparator: crate::datadogV1_20270101::model::Comparator, palette: String, value: f64) -> ThresholdFormattingRule {
        ThresholdFormattingRule {
            comparator,
            custom_bg_color: None,
            custom_fg_color: None,
            hide_value: None,
            highlight_row: None,
            image_url: None,
            metric: None,
            palette,
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
    
    pub fn highlight_row(mut self, value: bool) -> Self {
        self.highlight_row = Some(value);
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
    
}



impl<'de> Deserialize<'de> for ThresholdFormattingRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThresholdFormattingRuleVisitor;
        impl<'a> Visitor<'a> for ThresholdFormattingRuleVisitor {
            type Value = ThresholdFormattingRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comparator: Option<crate::datadogV1_20270101::model::Comparator> = None;
                let mut custom_bg_color: Option<String> = None;
                let mut custom_fg_color: Option<String> = None;
                let mut hide_value: Option<bool> = None;
                let mut highlight_row: Option<bool> = None;
                let mut image_url: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut palette: Option<String> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comparator" => {
                            comparator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _comparator) = comparator {
                                match _comparator {
                                    crate::datadogV1_20270101::model::Comparator::UnparsedObject(_comparator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "custom_bg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_bg_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "custom_fg_color" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_fg_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "hide_value" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "highlight_row" => {
                            if v.is_null() {
                                continue;
                            }
                            highlight_row = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "image_url" => {
                            if v.is_null() {
                                continue;
                            }
                            image_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "palette" => {
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let comparator = comparator.ok_or_else(|| M::Error::missing_field("comparator"))?;
                let palette = palette.ok_or_else(|| M::Error::missing_field("palette"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ThresholdFormattingRule {
                    comparator,
                    custom_bg_color,
                    custom_fg_color,
                    hide_value,
                    highlight_row,
                    image_url,
                    metric,
                    palette,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ThresholdFormattingRuleVisitor)
    }
}
