// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Host Metrics collected.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMetrics {
    /// The percent of CPU used (everything but idle).
    #[serde(rename = "cpu")]
    pub cpu: Option<f64>,
    /// The percent of CPU spent waiting on the IO (not reported for all platforms).
    #[serde(rename = "iowait")]
    pub iowait: Option<f64>,
    /// The system load over the last 15 minutes.
    #[serde(rename = "load")]
    pub load: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMetrics {
    pub fn new() -> HostMetrics {
        HostMetrics {
            cpu: None,
            iowait: None,
            load: None,
            _unparsed: false,
        }
    }

    pub fn cpu(mut self, value: f64) -> Self {
        self.cpu = Some(value);
        self
    }

    pub fn iowait(mut self, value: f64) -> Self {
        self.iowait = Some(value);
        self
    }

    pub fn load(mut self, value: f64) -> Self {
        self.load = Some(value);
        self
    }
}

impl Default for HostMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMetricsVisitor;
        impl<'a> Visitor<'a> for HostMetricsVisitor {
            type Value = HostMetrics;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cpu: Option<f64> = None;
                let mut iowait: Option<f64> = None;
                let mut load: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cpu" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iowait" => {
                            if v.is_null() {
                                continue;
                            }
                            iowait = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "load" => {
                            if v.is_null() {
                                continue;
                            }
                            load = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMetrics {
                    cpu,
                    iowait,
                    load,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMetricsVisitor)
    }
}
