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
pub struct HeatgridGradientConfig20270101OneOf4457883232 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemMode4447621888,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "preset_name")]
    pub preset_name: crate::datadogV1_20270101::model::HeatgridPresetName,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "source")]
    pub source: crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemSource4447622208,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatgridGradientConfig20270101OneOf4457883232 {
    pub fn new(mode: crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemMode4447621888, preset_name: crate::datadogV1_20270101::model::HeatgridPresetName, source: crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemSource4447622208) -> HeatgridGradientConfig20270101OneOf4457883232 {
        HeatgridGradientConfig20270101OneOf4457883232 {
            mode,
            preset_name,
            source,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for HeatgridGradientConfig20270101OneOf4457883232 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridGradientConfig20270101OneOf4457883232Visitor;
        impl<'a> Visitor<'a> for HeatgridGradientConfig20270101OneOf4457883232Visitor {
            type Value = HeatgridGradientConfig20270101OneOf4457883232;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemMode4447621888> = None;
                let mut preset_name: Option<crate::datadogV1_20270101::model::HeatgridPresetName> = None;
                let mut source: Option<crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemSource4447622208> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemMode4447621888::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "preset_name" => {
                            preset_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _preset_name) = preset_name {
                                match _preset_name {
                                    crate::datadogV1_20270101::model::HeatgridPresetName::UnparsedObject(_preset_name) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV1_20270101::model::HeatgridGradientConfigAnyOfAnyOfItemSource4447622208::UnparsedObject(_source) => {
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
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let preset_name = preset_name.ok_or_else(|| M::Error::missing_field("preset_name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = HeatgridGradientConfig20270101OneOf4457883232 {
                    mode,
                    preset_name,
                    source,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridGradientConfig20270101OneOf4457883232Visitor)
    }
}
