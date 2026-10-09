// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A color and optional lower threshold for a discrete bin.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridColorBin {
    /// A color string, or two color strings for the light and dark themes, in that order.
    #[serde(rename = "color")]
    pub color: crate::datadogV1::model::HeatgridColor,
    /// Inclusive lower bound. Omit for the first bin.
    #[serde(rename = "lower_bound")]
    pub lower_bound: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridColorBin {
    pub fn new(color: crate::datadogV1::model::HeatgridColor) -> HeatgridColorBin {
        HeatgridColorBin {
            color,
            lower_bound: None,
            _unparsed: false,
        }
    }

    pub fn lower_bound(mut self, value: f64) -> Self {
        self.lower_bound = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for HeatgridColorBin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridColorBinVisitor;
        impl<'a> Visitor<'a> for HeatgridColorBinVisitor {
            type Value = HeatgridColorBin;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color: Option<crate::datadogV1::model::HeatgridColor> = None;
                let mut lower_bound: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color" => {
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color) = color {
                                match _color {
                                    crate::datadogV1::model::HeatgridColor::UnparsedObject(
                                        _color,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "lower_bound" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            lower_bound =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let color = color.ok_or_else(|| M::Error::missing_field("color"))?;

                let content = HeatgridColorBin {
                    color,
                    lower_bound,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridColorBinVisitor)
    }
}
