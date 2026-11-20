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
pub struct HeatgridGradientConfig20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemMode,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "source")]
    pub source: crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "stops")]
    pub stops: Vec<crate::datadogV1::model::HeatgridStop>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatgridGradientConfig20270101OneOf {
    pub fn new(mode: crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemMode, source: crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemSource, stops: Vec<crate::datadogV1::model::HeatgridStop>) -> HeatgridGradientConfig20270101OneOf {
        HeatgridGradientConfig20270101OneOf {
            mode,
            source,
            stops,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for HeatgridGradientConfig20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatgridGradientConfig20270101OneOfVisitor;
        impl<'a> Visitor<'a> for HeatgridGradientConfig20270101OneOfVisitor {
            type Value = HeatgridGradientConfig20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mode: Option<crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemMode> = None;
                let mut source: Option<crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemSource> = None;
                let mut stops: Option<Vec<crate::datadogV1::model::HeatgridStop>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemMode::UnparsedObject(_mode) => {
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
                                    crate::datadogV1::model::HeatgridGradientConfigAnyOfAnyOfItemSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "stops" => {
                            stops = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let stops = stops.ok_or_else(|| M::Error::missing_field("stops"))?;

                let content = HeatgridGradientConfig20270101OneOf {
                    mode,
                    source,
                    stops,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatgridGradientConfig20270101OneOfVisitor)
    }
}
