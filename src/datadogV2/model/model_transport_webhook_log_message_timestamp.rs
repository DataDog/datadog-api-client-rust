// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The message delivery timing information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogMessageTimestamp {
    /// The Unix timestamp of the event.
    #[serde(rename = "event_timestamp")]
    pub event_timestamp: Option<f64>,
    /// The total delivery time in seconds.
    #[serde(rename = "lifetime")]
    pub lifetime: Option<f64>,
    /// Number of seconds the message spent in the delivery queue.
    #[serde(rename = "queue_time")]
    pub queue_time: Option<f64>,
    /// The scheduled delivery time as a Unix timestamp.
    #[serde(rename = "scheduled_time")]
    pub scheduled_time: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogMessageTimestamp {
    pub fn new() -> TransportWebhookLogMessageTimestamp {
        TransportWebhookLogMessageTimestamp {
            event_timestamp: None,
            lifetime: None,
            queue_time: None,
            scheduled_time: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn event_timestamp(mut self, value: f64) -> Self {
        self.event_timestamp = Some(value);
        self
    }

    pub fn lifetime(mut self, value: f64) -> Self {
        self.lifetime = Some(value);
        self
    }

    pub fn queue_time(mut self, value: f64) -> Self {
        self.queue_time = Some(value);
        self
    }

    pub fn scheduled_time(mut self, value: f64) -> Self {
        self.scheduled_time = Some(value);
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

impl Default for TransportWebhookLogMessageTimestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogMessageTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogMessageTimestampVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogMessageTimestampVisitor {
            type Value = TransportWebhookLogMessageTimestamp;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_timestamp: Option<f64> = None;
                let mut lifetime: Option<f64> = None;
                let mut queue_time: Option<f64> = None;
                let mut scheduled_time: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_timestamp" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            event_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lifetime" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            lifetime = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_time" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            queue_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scheduled_time" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            scheduled_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogMessageTimestamp {
                    event_timestamp,
                    lifetime,
                    queue_time,
                    scheduled_time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogMessageTimestampVisitor)
    }
}
