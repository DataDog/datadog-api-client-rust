// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A continuous gradient with custom color stops.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridGradientCustomColor {
    /// Use a continuous color gradient.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1::model::HeatgridGradientMode,
    /// Use custom colors.
    #[serde(rename = "source")]
    pub source: crate::datadogV1::model::HeatgridCustomColorSource,
    /// Two to six stops with positions in ascending order.
    #[serde(rename = "stops")]
    pub stops: Vec<crate::datadogV1::model::HeatgridColorStop>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridGradientCustomColor {
    pub fn new(
        mode: crate::datadogV1::model::HeatgridGradientMode,
        source: crate::datadogV1::model::HeatgridCustomColorSource,
        stops: Vec<crate::datadogV1::model::HeatgridColorStop>,
    ) -> HeatgridGradientCustomColor {
        HeatgridGradientCustomColor {
            mode,
            source,
            stops,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridGradientCustomColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridGradientCustomColorVisitor;
        impl<'a> Visitor<'a> for HeatgridGradientCustomColorVisitor {
            type Value = HeatgridGradientCustomColor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV1::model::HeatgridGradientMode> = None;
                let mut source: Option<crate::datadogV1::model::HeatgridCustomColorSource> = None;
                let mut stops: Option<Vec<crate::datadogV1::model::HeatgridColorStop>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1::model::HeatgridGradientMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV1::model::HeatgridCustomColorSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "stops" => {
                            stops = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let stops = stops.ok_or_else(|| M::Error::missing_field("stops"))?;

                let content = HeatgridGradientCustomColor {
                    mode,
                    source,
                    stops,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridGradientCustomColorVisitor)
    }
}
