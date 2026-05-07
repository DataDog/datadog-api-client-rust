// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The message identifiers.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogMessageId {
    /// The RFC 5322 Message-ID.
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
    /// The SMTP transaction identifier.
    #[serde(rename = "smtp_id")]
    pub smtp_id: Option<String>,
    /// The transport provider event identifier.
    #[serde(rename = "transport_event_id")]
    pub transport_event_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogMessageId {
    pub fn new() -> TransportWebhookLogMessageId {
        TransportWebhookLogMessageId {
            message_id: None,
            smtp_id: None,
            transport_event_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn message_id(mut self, value: String) -> Self {
        self.message_id = Some(value);
        self
    }

    pub fn smtp_id(mut self, value: String) -> Self {
        self.smtp_id = Some(value);
        self
    }

    pub fn transport_event_id(mut self, value: String) -> Self {
        self.transport_event_id = Some(value);
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

impl Default for TransportWebhookLogMessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogMessageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogMessageIdVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogMessageIdVisitor {
            type Value = TransportWebhookLogMessageId;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message_id: Option<String> = None;
                let mut smtp_id: Option<String> = None;
                let mut transport_event_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message_id" => {
                            if v.is_null() {
                                continue;
                            }
                            message_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "smtp_id" => {
                            if v.is_null() {
                                continue;
                            }
                            smtp_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "transport_event_id" => {
                            if v.is_null() {
                                continue;
                            }
                            transport_event_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogMessageId {
                    message_id,
                    smtp_id,
                    transport_event_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogMessageIdVisitor)
    }
}
