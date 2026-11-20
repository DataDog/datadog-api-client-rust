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
pub struct TimeseriesBackground {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::TimeseriesBackgroundType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1_20270101::model::Axis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesBackground {
    pub fn new(type_: crate::datadogV1_20270101::model::TimeseriesBackgroundType) -> TimeseriesBackground {
        TimeseriesBackground {
            type_,
            yaxis: None,
            _unparsed: false,
        }
    }
    
    pub fn yaxis(mut self, value: crate::datadogV1_20270101::model::Axis) -> Self {
        self.yaxis = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TimeseriesBackground {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesBackgroundVisitor;
        impl<'a> Visitor<'a> for TimeseriesBackgroundVisitor {
            type Value = TimeseriesBackground;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1_20270101::model::TimeseriesBackgroundType> = None;
                let mut yaxis: Option<crate::datadogV1_20270101::model::Axis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::TimeseriesBackgroundType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TimeseriesBackground {
                    type_,
                    yaxis,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesBackgroundVisitor)
    }
}
