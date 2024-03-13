// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Style customization for a top list widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ToplistWidgetStyle {
    /// Top list widget display options.
    #[serde(rename = "display")]
    pub display: Option<crate::datadogV1::model::ToplistWidgetDisplay>,
    /// Top list widget scaling definition.
    #[serde(rename = "scaling")]
    pub scaling: Option<crate::datadogV1::model::ToplistWidgetScaling>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ToplistWidgetStyle {
    pub fn new() -> ToplistWidgetStyle {
        ToplistWidgetStyle {
            display: None,
            scaling: None,
            _unparsed: false,
        }
    }

    pub fn display(mut self, value: crate::datadogV1::model::ToplistWidgetDisplay) -> Self {
        self.display = Some(value);
        self
    }

    pub fn scaling(mut self, value: crate::datadogV1::model::ToplistWidgetScaling) -> Self {
        self.scaling = Some(value);
        self
    }
}

impl Default for ToplistWidgetStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ToplistWidgetStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistWidgetStyleVisitor;
        impl<'a> Visitor<'a> for ToplistWidgetStyleVisitor {
            type Value = ToplistWidgetStyle;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display: Option<crate::datadogV1::model::ToplistWidgetDisplay> = None;
                let mut scaling: Option<crate::datadogV1::model::ToplistWidgetScaling> = None;
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
                                    crate::datadogV1::model::ToplistWidgetDisplay::UnparsedObject(_display) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "scaling" => {
                            if v.is_null() {
                                continue;
                            }
                            scaling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _scaling) = scaling {
                                match _scaling {
                                    crate::datadogV1::model::ToplistWidgetScaling::UnparsedObject(_scaling) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = ToplistWidgetStyle {
                    display,
                    scaling,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistWidgetStyleVisitor)
    }
}
