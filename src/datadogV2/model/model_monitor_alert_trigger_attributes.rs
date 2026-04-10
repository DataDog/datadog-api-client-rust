// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a monitor alert trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorAlertTriggerAttributes {
    /// The event ID associated with the monitor alert.
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// The timestamp of the event in Unix milliseconds.
    #[serde(rename = "event_ts")]
    pub event_ts: i64,
    /// The monitor ID that triggered the alert.
    #[serde(rename = "monitor_id")]
    pub monitor_id: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorAlertTriggerAttributes {
    pub fn new(event_id: String, event_ts: i64, monitor_id: i64) -> MonitorAlertTriggerAttributes {
        MonitorAlertTriggerAttributes {
            event_id,
            event_ts,
            monitor_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for MonitorAlertTriggerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorAlertTriggerAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorAlertTriggerAttributesVisitor {
            type Value = MonitorAlertTriggerAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_id: Option<String> = None;
                let mut event_ts: Option<i64> = None;
                let mut monitor_id: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_id" => {
                            event_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_ts" => {
                            event_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let event_id = event_id.ok_or_else(|| M::Error::missing_field("event_id"))?;
                let event_ts = event_ts.ok_or_else(|| M::Error::missing_field("event_ts"))?;
                let monitor_id = monitor_id.ok_or_else(|| M::Error::missing_field("monitor_id"))?;

                let content = MonitorAlertTriggerAttributes {
                    event_id,
                    event_ts,
                    monitor_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorAlertTriggerAttributesVisitor)
    }
}
