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
pub struct BarChartBackendDefinitionStyle {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101Display>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scaling")]
    pub scaling: Option<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyleScaling>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl BarChartBackendDefinitionStyle {
    pub fn new() -> BarChartBackendDefinitionStyle {
        BarChartBackendDefinitionStyle {
            display: None,
            palette: None,
            scaling: None,
            _unparsed: false,
        }
    }
    
    pub fn display(mut self, value: crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101Display) -> Self {
        self.display = Some(value);
        self
    }
    
    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }
    
    pub fn scaling(mut self, value: crate::datadogV1_20270101::model::BarChartBackendDefinitionStyleScaling) -> Self {
        self.scaling = Some(value);
        self
    }
    
}


impl Default for BarChartBackendDefinitionStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BarChartBackendDefinitionStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BarChartBackendDefinitionStyleVisitor;
        impl<'a> Visitor<'a> for BarChartBackendDefinitionStyleVisitor {
            type Value = BarChartBackendDefinitionStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display: Option<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101Display> = None;
                let mut palette: Option<String> = None;
                let mut scaling: Option<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyleScaling> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display" => {
                            if v.is_null() {
                                continue;
                            }
                            display = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display) = display {
                                match _display {
                                    crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101Display::UnparsedObject(_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "scaling" => {
                            if v.is_null() {
                                continue;
                            }
                            scaling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scaling) = scaling {
                                match _scaling {
                                    crate::datadogV1_20270101::model::BarChartBackendDefinitionStyleScaling::UnparsedObject(_scaling) => {
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

                let content = BarChartBackendDefinitionStyle {
                    display,
                    palette,
                    scaling,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BarChartBackendDefinitionStyleVisitor)
    }
}
