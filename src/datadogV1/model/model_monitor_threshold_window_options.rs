// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Alerting time window options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorThresholdWindowOptions {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    #[serde(
        rename = "recovery_window",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub recovery_window: Option<Option<String>>,
    /// Describes how long a metric must be anomalous before an alert triggers.
    #[serde(
        rename = "trigger_window",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub trigger_window: Option<Option<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorThresholdWindowOptions {
    pub fn new() -> MonitorThresholdWindowOptions {
        MonitorThresholdWindowOptions {
            recovery_window: None,
            trigger_window: None,
            _unparsed: false,
        }
    }

    pub fn recovery_window(mut self, value: Option<String>) -> Self {
        self.recovery_window = Some(value);
        self
    }

    pub fn trigger_window(mut self, value: Option<String>) -> Self {
        self.trigger_window = Some(value);
        self
    }
}

impl Default for MonitorThresholdWindowOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorThresholdWindowOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorThresholdWindowOptionsVisitor;
        impl<'a> Visitor<'a> for MonitorThresholdWindowOptionsVisitor {
            type Value = MonitorThresholdWindowOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recovery_window: Option<Option<String>> = None;
                let mut trigger_window: Option<Option<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "recovery_window" => {
                            recovery_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_window" => {
                            trigger_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorThresholdWindowOptions {
                    recovery_window,
                    trigger_window,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorThresholdWindowOptionsVisitor)
    }
}
