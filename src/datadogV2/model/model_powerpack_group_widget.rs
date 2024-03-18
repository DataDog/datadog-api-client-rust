// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Powerpack group widget definition object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackGroupWidget {
    /// Powerpack group widget object.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::PowerpackGroupWidgetDefinition,
    /// Powerpack group widget layout.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV2::model::PowerpackGroupWidgetLayout>,
    /// The available timeframes depend on the widget you are using.
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV2::model::WidgetLiveSpan>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackGroupWidget {
    pub fn new(
        definition: crate::datadogV2::model::PowerpackGroupWidgetDefinition,
    ) -> PowerpackGroupWidget {
        PowerpackGroupWidget {
            definition,
            layout: None,
            live_span: None,
            _unparsed: false,
        }
    }

    pub fn layout(mut self, value: crate::datadogV2::model::PowerpackGroupWidgetLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn live_span(mut self, value: crate::datadogV2::model::WidgetLiveSpan) -> Self {
        self.live_span = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackGroupWidget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackGroupWidgetVisitor;
        impl<'a> Visitor<'a> for PowerpackGroupWidgetVisitor {
            type Value = PowerpackGroupWidget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<
                    crate::datadogV2::model::PowerpackGroupWidgetDefinition,
                > = None;
                let mut layout: Option<crate::datadogV2::model::PowerpackGroupWidgetLayout> = None;
                let mut live_span: Option<crate::datadogV2::model::WidgetLiveSpan> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "layout" => {
                            if v.is_null() {
                                continue;
                            }
                            layout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live_span" => {
                            if v.is_null() {
                                continue;
                            }
                            live_span = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _live_span) = live_span {
                                match _live_span {
                                    crate::datadogV2::model::WidgetLiveSpan::UnparsedObject(
                                        _live_span,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;

                let content = PowerpackGroupWidget {
                    definition,
                    layout,
                    live_span,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackGroupWidgetVisitor)
    }
}
