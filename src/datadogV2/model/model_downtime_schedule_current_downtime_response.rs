// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The most recent actual start and end dates for a recurring downtime. For a canceled downtime,
/// this is the previously occurring downtime. For active downtimes, this is the ongoing downtime, and for scheduled
/// downtimes it is the upcoming downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleCurrentDowntimeResponse {
    /// The end of the current downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The start of the current downtime.
    #[serde(rename = "start")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleCurrentDowntimeResponse {
    pub fn new() -> DowntimeScheduleCurrentDowntimeResponse {
        DowntimeScheduleCurrentDowntimeResponse {
            end: None,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn start(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start = Some(value);
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

impl Default for DowntimeScheduleCurrentDowntimeResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleCurrentDowntimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleCurrentDowntimeResponseVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleCurrentDowntimeResponseVisitor {
            type Value = DowntimeScheduleCurrentDowntimeResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut start: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeScheduleCurrentDowntimeResponse {
                    end,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleCurrentDowntimeResponseVisitor)
    }
}
