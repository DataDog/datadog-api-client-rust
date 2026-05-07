// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The message delivery event details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogMessage {
    /// The message authentication details.
    #[serde(rename = "auth")]
    pub auth: Option<crate::datadogV2::model::TransportWebhookLogMessageAuth>,
    /// Custom arguments passed through the email transport provider for tracking.
    #[serde(rename = "custom_args")]
    pub custom_args: Option<crate::datadogV2::model::TransportWebhookLogMessageCustomArgs>,
    /// The message identifiers.
    #[serde(rename = "id")]
    pub id: Option<crate::datadogV2::model::TransportWebhookLogMessageId>,
    /// The delivery event type emitted by the transport provider (for example, "delivered", "dropped", "bounced").
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The SMTP response information.
    #[serde(rename = "response")]
    pub response: Option<crate::datadogV2::model::TransportWebhookLogMessageResponse>,
    /// The IP address of the sending server.
    #[serde(rename = "sender_ip")]
    pub sender_ip: Option<String>,
    /// The message delivery timing information.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<crate::datadogV2::model::TransportWebhookLogMessageTimestamp>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogMessage {
    pub fn new() -> TransportWebhookLogMessage {
        TransportWebhookLogMessage {
            auth: None,
            custom_args: None,
            id: None,
            name: None,
            response: None,
            sender_ip: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn auth(mut self, value: crate::datadogV2::model::TransportWebhookLogMessageAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn custom_args(
        mut self,
        value: crate::datadogV2::model::TransportWebhookLogMessageCustomArgs,
    ) -> Self {
        self.custom_args = Some(value);
        self
    }

    pub fn id(mut self, value: crate::datadogV2::model::TransportWebhookLogMessageId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn response(
        mut self,
        value: crate::datadogV2::model::TransportWebhookLogMessageResponse,
    ) -> Self {
        self.response = Some(value);
        self
    }

    pub fn sender_ip(mut self, value: String) -> Self {
        self.sender_ip = Some(value);
        self
    }

    pub fn timestamp(
        mut self,
        value: crate::datadogV2::model::TransportWebhookLogMessageTimestamp,
    ) -> Self {
        self.timestamp = Some(value);
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

impl Default for TransportWebhookLogMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogMessageVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogMessageVisitor {
            type Value = TransportWebhookLogMessage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::TransportWebhookLogMessageAuth> =
                    None;
                let mut custom_args: Option<
                    crate::datadogV2::model::TransportWebhookLogMessageCustomArgs,
                > = None;
                let mut id: Option<crate::datadogV2::model::TransportWebhookLogMessageId> = None;
                let mut name: Option<String> = None;
                let mut response: Option<
                    crate::datadogV2::model::TransportWebhookLogMessageResponse,
                > = None;
                let mut sender_ip: Option<String> = None;
                let mut timestamp: Option<
                    crate::datadogV2::model::TransportWebhookLogMessageTimestamp,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            if v.is_null() {
                                continue;
                            }
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_args" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_args =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response" => {
                            if v.is_null() {
                                continue;
                            }
                            response = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sender_ip" => {
                            if v.is_null() {
                                continue;
                            }
                            sender_ip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogMessage {
                    auth,
                    custom_args,
                    id,
                    name,
                    response,
                    sender_ip,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogMessageVisitor)
    }
}
