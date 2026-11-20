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
pub struct SunburstLegend20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_percent")]
    pub hide_percent: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstLegendAnyOfAnyOfItemType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SunburstLegend20270101OneOf {
    pub fn new(type_: crate::datadogV1::model::SunburstLegendAnyOfAnyOfItemType) -> SunburstLegend20270101OneOf {
        SunburstLegend20270101OneOf {
            hide_percent: None,
            hide_value: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn hide_percent(mut self, value: bool) -> Self {
        self.hide_percent = Some(value);
        self
    }
    
    pub fn hide_value(mut self, value: bool) -> Self {
        self.hide_value = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for SunburstLegend20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SunburstLegend20270101OneOfVisitor;
        impl<'a> Visitor<'a> for SunburstLegend20270101OneOfVisitor {
            type Value = SunburstLegend20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_percent: Option<bool> = None;
                let mut hide_value: Option<bool> = None;
                let mut type_: Option<crate::datadogV1::model::SunburstLegendAnyOfAnyOfItemType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_percent" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_percent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "hide_value" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SunburstLegendAnyOfAnyOfItemType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SunburstLegend20270101OneOf {
                    hide_percent,
                    hide_value,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SunburstLegend20270101OneOfVisitor)
    }
}
