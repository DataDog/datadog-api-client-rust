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
pub struct WidgetTime20270101OneOf4464103552 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_incomplete_cost_data")]
    pub hide_incomplete_cost_data: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::WidgetNewLiveSpanType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unit")]
    pub unit: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl WidgetTime20270101OneOf4464103552 {
    pub fn new(type_: crate::datadogV1::model::WidgetNewLiveSpanType, unit: String, value: f64) -> WidgetTime20270101OneOf4464103552 {
        WidgetTime20270101OneOf4464103552 {
            hide_incomplete_cost_data: None,
            type_,
            unit,
            value,
            _unparsed: false,
        }
    }
    
    pub fn hide_incomplete_cost_data(mut self, value: bool) -> Self {
        self.hide_incomplete_cost_data = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for WidgetTime20270101OneOf4464103552 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetTime20270101OneOf4464103552Visitor;
        impl<'a> Visitor<'a> for WidgetTime20270101OneOf4464103552Visitor {
            type Value = WidgetTime20270101OneOf4464103552;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_incomplete_cost_data: Option<bool> = None;
                let mut type_: Option<crate::datadogV1::model::WidgetNewLiveSpanType> = None;
                let mut unit: Option<String> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_incomplete_cost_data" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_incomplete_cost_data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::WidgetNewLiveSpanType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = WidgetTime20270101OneOf4464103552 {
                    hide_incomplete_cost_data,
                    type_,
                    unit,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetTime20270101OneOf4464103552Visitor)
    }
}
