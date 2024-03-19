// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// X Axis controls for the distribution widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionWidgetXAxis {
    /// True includes zero.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// Specifies maximum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Specifies minimum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear`.
    #[serde(rename = "scale")]
    pub scale: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DistributionWidgetXAxis {
    pub fn new() -> DistributionWidgetXAxis {
        DistributionWidgetXAxis {
            include_zero: None,
            max: None,
            min: None,
            scale: None,
            _unparsed: false,
        }
    }

    pub fn include_zero(mut self, value: bool) -> Self {
        self.include_zero = Some(value);
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

    pub fn scale(mut self, value: String) -> Self {
        self.scale = Some(value);
        self
    }
}

impl Default for DistributionWidgetXAxis {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DistributionWidgetXAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionWidgetXAxisVisitor;
        impl<'a> Visitor<'a> for DistributionWidgetXAxisVisitor {
            type Value = DistributionWidgetXAxis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_zero: Option<bool> = None;
                let mut max: Option<String> = None;
                let mut min: Option<String> = None;
                let mut scale: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_zero" => {
                            if v.is_null() {
                                continue;
                            }
                            include_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "scale" => {
                            if v.is_null() {
                                continue;
                            }
                            scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DistributionWidgetXAxis {
                    include_zero,
                    max,
                    min,
                    scale,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionWidgetXAxisVisitor)
    }
}
