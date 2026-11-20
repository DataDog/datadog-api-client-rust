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
pub struct Marker {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_type")]
    pub display_type: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "from_ts")]
    pub from_ts: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_hovered")]
    pub is_hovered: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "max")]
    pub max: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "min")]
    pub min: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time")]
    pub time: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "to_ts")]
    pub to_ts: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "ts")]
    pub ts: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "val")]
    pub val: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl Marker {
    pub fn new() -> Marker {
        Marker {
            display_type: None,
            from_ts: None,
            is_hovered: None,
            label: None,
            max: None,
            min: None,
            on_right_yaxis: None,
            time: None,
            to_ts: None,
            ts: None,
            type_: None,
            val: None,
            value: None,
            _unparsed: false,
        }
    }
    
    pub fn display_type(mut self, value: String) -> Self {
        self.display_type = Some(value);
        self
    }
    
    pub fn from_ts(mut self, value: String) -> Self {
        self.from_ts = Some(value);
        self
    }
    
    pub fn is_hovered(mut self, value: bool) -> Self {
        self.is_hovered = Some(value);
        self
    }
    
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    
    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }
    
    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
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
    
    pub fn to_ts(mut self, value: String) -> Self {
        self.to_ts = Some(value);
        self
    }
    
    pub fn ts(mut self, value: String) -> Self {
        self.ts = Some(value);
        self
    }
    
    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
    
    pub fn val(mut self, value: f64) -> Self {
        self.val = Some(value);
        self
    }
    
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
    
}


impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Marker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MarkerVisitor;
        impl<'a> Visitor<'a> for MarkerVisitor {
            type Value = Marker;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_type: Option<String> = None;
                let mut from_ts: Option<String> = None;
                let mut is_hovered: Option<bool> = None;
                let mut label: Option<String> = None;
                let mut max: Option<f64> = None;
                let mut min: Option<f64> = None;
                let mut on_right_yaxis: Option<bool> = None;
                let mut time: Option<String> = None;
                let mut to_ts: Option<String> = None;
                let mut ts: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut val: Option<f64> = None;
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
                        "from_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_hovered" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hovered = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "max" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "min" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            min = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "to_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "ts" => {
                            if v.is_null() {
                                continue;
                            }
                            ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "val" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            val = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = Marker {
                    display_type,
                    from_ts,
                    is_hovered,
                    label,
                    max,
                    min,
                    on_right_yaxis,
                    time,
                    to_ts,
                    ts,
                    type_,
                    val,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MarkerVisitor)
    }
}
