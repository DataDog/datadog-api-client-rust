// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration options for the custom schedule. **This feature is in private beta.**
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorOptionsCustomSchedule {
    /// Array of custom schedule recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Option<Vec<crate::datadogV1::model::MonitorOptionsCustomScheduleRecurrence>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorOptionsCustomSchedule {
    pub fn new() -> MonitorOptionsCustomSchedule {
        MonitorOptionsCustomSchedule {
            recurrences: None,
            _unparsed: false,
        }
    }

    pub fn recurrences(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorOptionsCustomScheduleRecurrence>,
    ) -> Self {
        self.recurrences = Some(value);
        self
    }
}

impl Default for MonitorOptionsCustomSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorOptionsCustomSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorOptionsCustomScheduleVisitor;
        impl<'a> Visitor<'a> for MonitorOptionsCustomScheduleVisitor {
            type Value = MonitorOptionsCustomSchedule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recurrences: Option<
                    Vec<crate::datadogV1::model::MonitorOptionsCustomScheduleRecurrence>,
                > = None;
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
                        &_ => {}
                    }
                }

                let content = MonitorOptionsCustomSchedule {
                    recurrences,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorOptionsCustomScheduleVisitor)
    }
}
