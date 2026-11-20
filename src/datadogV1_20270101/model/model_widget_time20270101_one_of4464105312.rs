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
pub struct WidgetTime20270101OneOf4464105312 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_incomplete_cost_data")]
    pub hide_incomplete_cost_data: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "live_span")]
    pub live_span: crate::datadogV1_20270101::model::LegacyLiveSpan,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl WidgetTime20270101OneOf4464105312 {
    pub fn new(live_span: crate::datadogV1_20270101::model::LegacyLiveSpan) -> WidgetTime20270101OneOf4464105312 {
        WidgetTime20270101OneOf4464105312 {
            hide_incomplete_cost_data: None,
            live_span,
            _unparsed: false,
        }
    }
    
    pub fn hide_incomplete_cost_data(mut self, value: bool) -> Self {
        self.hide_incomplete_cost_data = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for WidgetTime20270101OneOf4464105312 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetTime20270101OneOf4464105312Visitor;
        impl<'a> Visitor<'a> for WidgetTime20270101OneOf4464105312Visitor {
            type Value = WidgetTime20270101OneOf4464105312;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_incomplete_cost_data: Option<bool> = None;
                let mut live_span: Option<crate::datadogV1_20270101::model::LegacyLiveSpan> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_incomplete_cost_data" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_incomplete_cost_data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "live_span" => {
                            live_span = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _live_span) = live_span {
                                match _live_span {
                                    crate::datadogV1_20270101::model::LegacyLiveSpan::UnparsedObject(_live_span) => {
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
                let live_span = live_span.ok_or_else(|| M::Error::missing_field("live_span"))?;

                let content = WidgetTime20270101OneOf4464105312 {
                    hide_incomplete_cost_data,
                    live_span,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetTime20270101OneOf4464105312Visitor)
    }
}
