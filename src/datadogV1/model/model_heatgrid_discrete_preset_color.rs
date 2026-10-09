// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A preset discrete color palette.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatgridDiscretePresetColor {
    /// Use discrete color thresholds.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1::model::HeatgridDiscreteMode,
    /// Name of the preset color palette.
    #[serde(rename = "preset_name")]
    pub preset_name: String,
    /// Use a preset color palette.
    #[serde(rename = "source")]
    pub source: crate::datadogV1::model::HeatgridPresetColorSource,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HeatgridDiscretePresetColor {
    pub fn new(
        mode: crate::datadogV1::model::HeatgridDiscreteMode,
        preset_name: String,
        source: crate::datadogV1::model::HeatgridPresetColorSource,
    ) -> HeatgridDiscretePresetColor {
        HeatgridDiscretePresetColor {
            mode,
            preset_name,
            source,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for HeatgridDiscretePresetColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridDiscretePresetColorVisitor;
        impl<'a> Visitor<'a> for HeatgridDiscretePresetColorVisitor {
            type Value = HeatgridDiscretePresetColor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV1::model::HeatgridDiscreteMode> = None;
                let mut preset_name: Option<String> = None;
                let mut source: Option<crate::datadogV1::model::HeatgridPresetColorSource> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "preset_name" => {
                            preset_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV1::model::HeatgridPresetColorSource::UnparsedObject(_source) => {
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
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let preset_name =
                    preset_name.ok_or_else(|| M::Error::missing_field("preset_name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = HeatgridDiscretePresetColor {
                    mode,
                    preset_name,
                    source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridDiscretePresetColorVisitor)
    }
}
