// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A recurring downtime schedule definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleRecurrencesUpdateRequest {
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences:
        Option<Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>>,
    /// The timezone in which to schedule the downtime.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleRecurrencesUpdateRequest {
    pub fn new() -> DowntimeScheduleRecurrencesUpdateRequest {
        DowntimeScheduleRecurrencesUpdateRequest {
            recurrences: None,
            timezone: None,
            _unparsed: false,
        }
    }

    pub fn recurrences(
        mut self,
        value: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
    ) -> Self {
        self.recurrences = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl Default for DowntimeScheduleRecurrencesUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleRecurrencesUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleRecurrencesUpdateRequestVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleRecurrencesUpdateRequestVisitor {
            type Value = DowntimeScheduleRecurrencesUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recurrences: Option<
                    Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
                > = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "recurrences" => {
                            if v.is_null() {
                                continue;
                            }
                            recurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            _unparsed = true;
                        }
                    }
                }

                let content = DowntimeScheduleRecurrencesUpdateRequest {
                    recurrences,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleRecurrencesUpdateRequestVisitor)
    }
}
