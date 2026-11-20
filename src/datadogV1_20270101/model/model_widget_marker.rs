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
pub struct WidgetMarker {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_type")]
    pub display_type: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time")]
    pub time: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl WidgetMarker {
    pub fn new(value: String) -> WidgetMarker {
        WidgetMarker {
            display_type: None,
            label: None,
            on_right_yaxis: None,
            time: None,
            value,
            _unparsed: false,
        }
    }
    
    pub fn display_type(mut self, value: String) -> Self {
        self.display_type = Some(value);
        self
    }
    
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    
    pub fn on_right_yaxis(mut self, value: bool) -> Self {
        self.on_right_yaxis = Some(value);
        self
    }
    
    pub fn time(mut self, value: String) -> Self {
        self.time = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for WidgetMarker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetMarkerVisitor;
        impl<'a> Visitor<'a> for WidgetMarkerVisitor {
            type Value = WidgetMarker;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_type: Option<String> = None;
                let mut label: Option<String> = None;
                let mut on_right_yaxis: Option<bool> = None;
                let mut time: Option<String> = None;
                let mut value: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "on_right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            on_right_yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = WidgetMarker {
                    display_type,
                    label,
                    on_right_yaxis,
                    time,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetMarkerVisitor)
    }
}
