// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A recurring downtime schedule definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleRecurrencesResponse {
    /// The most recent actual start and end dates for a recurring downtime. For a canceled downtime,
    /// this is the previously occurring downtime. For active downtimes, this is the ongoing downtime, and for scheduled
    /// downtimes it is the upcoming downtime.
    #[serde(rename = "current_downtime")]
    pub current_downtime: Option<crate::datadogV2::model::DowntimeScheduleCurrentDowntimeResponse>,
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceResponse>,
    /// The timezone in which to schedule the downtime. This affects recurring start and end dates.
    /// Must match `display_timezone`.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleRecurrencesResponse {
    pub fn new(
        recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceResponse>,
    ) -> DowntimeScheduleRecurrencesResponse {
        DowntimeScheduleRecurrencesResponse {
            current_downtime: None,
            recurrences,
            timezone: None,
            _unparsed: false,
        }
    }

    pub fn current_downtime(
        mut self,
        value: crate::datadogV2::model::DowntimeScheduleCurrentDowntimeResponse,
    ) -> Self {
        self.current_downtime = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleRecurrencesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleRecurrencesResponseVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleRecurrencesResponseVisitor {
            type Value = DowntimeScheduleRecurrencesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_downtime: Option<
                    crate::datadogV2::model::DowntimeScheduleCurrentDowntimeResponse,
                > = None;
                let mut recurrences: Option<
                    Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceResponse>,
                > = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "current_downtime" => {
                            if v.is_null() {
                                continue;
                            }
                            current_downtime =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recurrences" => {
                            recurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let recurrences =
                    recurrences.ok_or_else(|| M::Error::missing_field("recurrences"))?;

                let content = DowntimeScheduleRecurrencesResponse {
                    current_downtime,
                    recurrences,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleRecurrencesResponseVisitor)
    }
}
