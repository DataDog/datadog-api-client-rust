// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom arguments passed through the email transport provider for tracking.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogMessageCustomArgs {
    /// The unique email identifier.
    #[serde(rename = "email_id")]
    pub email_id: Option<String>,
    /// The human-readable email type name.
    #[serde(rename = "email_type_display_name")]
    pub email_type_display_name: Option<String>,
    /// The organization UUID.
    #[serde(rename = "org_uuid")]
    pub org_uuid: Option<String>,
    /// The timestamp when the email was queued.
    #[serde(rename = "queue_time")]
    pub queue_time: Option<String>,
    /// The email subject line.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogMessageCustomArgs {
    pub fn new() -> TransportWebhookLogMessageCustomArgs {
        TransportWebhookLogMessageCustomArgs {
            email_id: None,
            email_type_display_name: None,
            org_uuid: None,
            queue_time: None,
            subject: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn email_id(mut self, value: String) -> Self {
        self.email_id = Some(value);
        self
    }

    pub fn email_type_display_name(mut self, value: String) -> Self {
        self.email_type_display_name = Some(value);
        self
    }

    pub fn org_uuid(mut self, value: String) -> Self {
        self.org_uuid = Some(value);
        self
    }

    pub fn queue_time(mut self, value: String) -> Self {
        self.queue_time = Some(value);
        self
    }

    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
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

impl Default for TransportWebhookLogMessageCustomArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogMessageCustomArgs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogMessageCustomArgsVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogMessageCustomArgsVisitor {
            type Value = TransportWebhookLogMessageCustomArgs;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email_id: Option<String> = None;
                let mut email_type_display_name: Option<String> = None;
                let mut org_uuid: Option<String> = None;
                let mut queue_time: Option<String> = None;
                let mut subject: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email_id" => {
                            if v.is_null() {
                                continue;
                            }
                            email_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email_type_display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            email_type_display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            org_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queue_time" => {
                            if v.is_null() {
                                continue;
                            }
                            queue_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogMessageCustomArgs {
                    email_id,
                    email_type_display_name,
                    org_uuid,
                    queue_time,
                    subject,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogMessageCustomArgsVisitor)
    }
}
