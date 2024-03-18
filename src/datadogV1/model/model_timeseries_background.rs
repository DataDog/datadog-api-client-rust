// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Set a timeseries on the widget background.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesBackground {
    /// Timeseries is made using an area or bars.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TimeseriesBackgroundType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::WidgetAxis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesBackground {
    pub fn new(type_: crate::datadogV1::model::TimeseriesBackgroundType) -> TimeseriesBackground {
        TimeseriesBackground {
            type_,
            yaxis: None,
            _unparsed: false,
        }
    }

    pub fn yaxis(mut self, value: crate::datadogV1::model::WidgetAxis) -> Self {
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
                let mut type_: Option<crate::datadogV1::model::TimeseriesBackgroundType> = None;
                let mut yaxis: Option<crate::datadogV1::model::WidgetAxis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::TimeseriesBackgroundType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
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
