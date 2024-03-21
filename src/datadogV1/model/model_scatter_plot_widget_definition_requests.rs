// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Widget definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterPlotWidgetDefinitionRequests {
    /// Scatterplot request containing formulas and functions.
    #[serde(rename = "table")]
    pub table: Option<crate::datadogV1::model::ScatterplotTableRequest>,
    /// Updated scatter plot.
    #[serde(rename = "x")]
    pub x: Option<crate::datadogV1::model::ScatterPlotRequest>,
    /// Updated scatter plot.
    #[serde(rename = "y")]
    pub y: Option<crate::datadogV1::model::ScatterPlotRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScatterPlotWidgetDefinitionRequests {
    pub fn new() -> ScatterPlotWidgetDefinitionRequests {
        ScatterPlotWidgetDefinitionRequests {
            table: None,
            x: None,
            y: None,
            _unparsed: false,
        }
    }

    pub fn table(mut self, value: crate::datadogV1::model::ScatterplotTableRequest) -> Self {
        self.table = Some(value);
        self
    }

    pub fn x(mut self, value: crate::datadogV1::model::ScatterPlotRequest) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: crate::datadogV1::model::ScatterPlotRequest) -> Self {
        self.y = Some(value);
        self
    }
}

impl Default for ScatterPlotWidgetDefinitionRequests {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScatterPlotWidgetDefinitionRequests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterPlotWidgetDefinitionRequestsVisitor;
        impl<'a> Visitor<'a> for ScatterPlotWidgetDefinitionRequestsVisitor {
            type Value = ScatterPlotWidgetDefinitionRequests;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut table: Option<crate::datadogV1::model::ScatterplotTableRequest> = None;
                let mut x: Option<crate::datadogV1::model::ScatterPlotRequest> = None;
                let mut y: Option<crate::datadogV1::model::ScatterPlotRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "table" => {
                            if v.is_null() {
                                continue;
                            }
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "x" => {
                            if v.is_null() {
                                continue;
                            }
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "y" => {
                            if v.is_null() {
                                continue;
                            }
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ScatterPlotWidgetDefinitionRequests {
                    table,
                    x,
                    y,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterPlotWidgetDefinitionRequestsVisitor)
    }
}
