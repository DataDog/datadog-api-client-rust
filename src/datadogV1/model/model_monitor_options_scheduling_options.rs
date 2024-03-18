// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration options for scheduling.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorOptionsSchedulingOptions {
    /// Configuration options for the custom schedule. **This feature is in private beta.**
    #[serde(rename = "custom_schedule")]
    pub custom_schedule: Option<crate::datadogV1::model::MonitorOptionsCustomSchedule>,
    /// Configuration options for the evaluation window. If `hour_starts` is set, no other fields may be set. Otherwise, `day_starts` and `month_starts` must be set together.
    #[serde(rename = "evaluation_window")]
    pub evaluation_window:
        Option<crate::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorOptionsSchedulingOptions {
    pub fn new() -> MonitorOptionsSchedulingOptions {
        MonitorOptionsSchedulingOptions {
            custom_schedule: None,
            evaluation_window: None,
            _unparsed: false,
        }
    }

    pub fn custom_schedule(
        mut self,
        value: crate::datadogV1::model::MonitorOptionsCustomSchedule,
    ) -> Self {
        self.custom_schedule = Some(value);
        self
    }

    pub fn evaluation_window(
        mut self,
        value: crate::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow,
    ) -> Self {
        self.evaluation_window = Some(value);
        self
    }
}

impl Default for MonitorOptionsSchedulingOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorOptionsSchedulingOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorOptionsSchedulingOptionsVisitor;
        impl<'a> Visitor<'a> for MonitorOptionsSchedulingOptionsVisitor {
            type Value = MonitorOptionsSchedulingOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_schedule: Option<
                    crate::datadogV1::model::MonitorOptionsCustomSchedule,
                > = None;
                let mut evaluation_window: Option<
                    crate::datadogV1::model::MonitorOptionsSchedulingOptionsEvaluationWindow,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_schedule" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_schedule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation_window" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorOptionsSchedulingOptions {
                    custom_schedule,
                    evaluation_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorOptionsSchedulingOptionsVisitor)
    }
}
