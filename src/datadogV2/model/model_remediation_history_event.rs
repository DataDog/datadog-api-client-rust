// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single event in the investigation history.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationHistoryEvent {
    /// The type of event.
    #[serde(rename = "event_type")]
    pub event_type: Option<String>,
    /// History event ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Opaque JSON event body, base64-encoded. Decode and parse according to event_type.
    #[serde(rename = "payload")]
    pub payload: Option<String>,
    /// Event time in epoch milliseconds (64-bit integer encoded as a string).
    #[serde(rename = "timestamp_ms")]
    pub timestamp_ms: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationHistoryEvent {
    pub fn new() -> RemediationHistoryEvent {
        RemediationHistoryEvent {
            event_type: None,
            id: None,
            payload: None,
            timestamp_ms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn event_type(mut self, value: String) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn payload(mut self, value: String) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn timestamp_ms(mut self, value: String) -> Self {
        self.timestamp_ms = Some(value);
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

impl Default for RemediationHistoryEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationHistoryEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationHistoryEventVisitor;
        impl<'a> Visitor<'a> for RemediationHistoryEventVisitor {
            type Value = RemediationHistoryEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_type: Option<String> = None;
                let mut id: Option<String> = None;
                let mut payload: Option<String> = None;
                let mut timestamp_ms: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_type" => {
                            if v.is_null() {
                                continue;
                            }
                            event_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            if v.is_null() {
                                continue;
                            }
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationHistoryEvent {
                    event_type,
                    id,
                    payload,
                    timestamp_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationHistoryEventVisitor)
    }
}
