// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Downtime duration attributes of a monitor configuration policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorConfigPolicyDowntimePolicy {
    /// The maximum allowed downtime duration, in milliseconds.
    #[serde(rename = "max_duration_ms")]
    pub max_duration_ms: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorConfigPolicyDowntimePolicy {
    pub fn new(max_duration_ms: i64) -> MonitorConfigPolicyDowntimePolicy {
        MonitorConfigPolicyDowntimePolicy {
            max_duration_ms,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorConfigPolicyDowntimePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorConfigPolicyDowntimePolicyVisitor;
        impl<'a> Visitor<'a> for MonitorConfigPolicyDowntimePolicyVisitor {
            type Value = MonitorConfigPolicyDowntimePolicy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_duration_ms: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_duration_ms" => {
                            max_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let max_duration_ms =
                    max_duration_ms.ok_or_else(|| M::Error::missing_field("max_duration_ms"))?;

                let content = MonitorConfigPolicyDowntimePolicy {
                    max_duration_ms,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorConfigPolicyDowntimePolicyVisitor)
    }
}
