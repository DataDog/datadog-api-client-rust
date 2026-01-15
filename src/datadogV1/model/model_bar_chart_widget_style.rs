// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Style customization for a bar chart widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BarChartWidgetStyle {
    /// Bar chart widget display options.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV1::model::BarChartWidgetDisplay>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
    /// Bar chart widget scaling definition.
    #[serde(rename = "scaling")]
    pub scaling: Option<crate::datadogV1::model::BarChartWidgetScaling>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BarChartWidgetStyle {
    pub fn new() -> BarChartWidgetStyle {
        BarChartWidgetStyle {
            display: None,
            palette: None,
            scaling: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display(mut self, value: crate::datadogV1::model::BarChartWidgetDisplay) -> Self {
        self.display = Some(value);
        self
    }

    pub fn palette(mut self, value: String) -> Self {
        self.palette = Some(value);
        self
    }

    pub fn scaling(mut self, value: crate::datadogV1::model::BarChartWidgetScaling) -> Self {
        self.scaling = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for BarChartWidgetStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BarChartWidgetStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BarChartWidgetStyleVisitor;
        impl<'a> Visitor<'a> for BarChartWidgetStyleVisitor {
            type Value = BarChartWidgetStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display: Option<crate::datadogV1::model::BarChartWidgetDisplay> = None;
                let mut palette: Option<String> = None;
                let mut scaling: Option<crate::datadogV1::model::BarChartWidgetScaling> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                                    crate::datadogV1::model::BarChartWidgetDisplay::UnparsedObject(_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "palette" => {
                            if v.is_null() {
                                continue;
                            }
                            palette = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scaling" => {
                            if v.is_null() {
                                continue;
                            }
                            scaling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scaling) = scaling {
                                match _scaling {
                                    crate::datadogV1::model::BarChartWidgetScaling::UnparsedObject(_scaling) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BarChartWidgetStyle {
                    display,
                    palette,
                    scaling,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BarChartWidgetStyleVisitor)
    }
}
