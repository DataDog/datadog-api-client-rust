// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger a workflow VIA a Monitor. For automatic triggering a handle must be configured and the workflow must be published.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorTrigger {
    /// Defines a rate limit for a trigger.
    #[serde(rename = "rateLimit")]
    pub rate_limit: Option<crate::datadogV2::model::TriggerRateLimit>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorTrigger {
    pub fn new() -> MonitorTrigger {
        MonitorTrigger {
            rate_limit: None,
            _unparsed: false,
        }
    }

    pub fn rate_limit(mut self, value: crate::datadogV2::model::TriggerRateLimit) -> Self {
        self.rate_limit = Some(value);
        self
    }
}

impl Default for MonitorTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorTriggerVisitor;
        impl<'a> Visitor<'a> for MonitorTriggerVisitor {
            type Value = MonitorTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rate_limit: Option<crate::datadogV2::model::TriggerRateLimit> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rateLimit" => {
                            if v.is_null() {
                                continue;
                            }
                            rate_limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = MonitorTrigger {
                    rate_limit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorTriggerVisitor)
    }
}
