// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Discrete thresholds with custom colors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridDiscreteCustomColor {
    /// Two to six bins. Omit `lower_bound` on the first bin. Subsequent lower bounds must be in
    /// ascending order.
    #[serde(rename = "bins")]
    pub bins: Vec<crate::datadogV1::model::HeatgridColorBin>,
    /// Use discrete color thresholds.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1::model::HeatgridDiscreteMode,
    /// Use custom colors.
    #[serde(rename = "source")]
    pub source: crate::datadogV1::model::HeatgridCustomColorSource,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridDiscreteCustomColor {
    pub fn new(
        bins: Vec<crate::datadogV1::model::HeatgridColorBin>,
        mode: crate::datadogV1::model::HeatgridDiscreteMode,
        source: crate::datadogV1::model::HeatgridCustomColorSource,
    ) -> HeatgridDiscreteCustomColor {
        HeatgridDiscreteCustomColor {
            bins,
            mode,
            source,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridDiscreteCustomColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridDiscreteCustomColorVisitor;
        impl<'a> Visitor<'a> for HeatgridDiscreteCustomColorVisitor {
            type Value = HeatgridDiscreteCustomColor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bins: Option<Vec<crate::datadogV1::model::HeatgridColorBin>> = None;
                let mut mode: Option<crate::datadogV1::model::HeatgridDiscreteMode> = None;
                let mut source: Option<crate::datadogV1::model::HeatgridCustomColorSource> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bins" => {
                            bins = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1::model::HeatgridDiscreteMode::UnparsedObject(_mode) => {
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
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let bins = bins.ok_or_else(|| M::Error::missing_field("bins"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = HeatgridDiscreteCustomColor {
                    bins,
                    mode,
                    source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridDiscreteCustomColorVisitor)
    }
}
