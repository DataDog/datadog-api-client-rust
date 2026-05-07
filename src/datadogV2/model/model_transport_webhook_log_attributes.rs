// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Top-level attributes for the webhook log event, including delivery status, recipient details, and provider metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLogAttributes {
    /// The event categories.
    #[serde(rename = "category")]
    pub category: Option<Vec<String>>,
    /// The email address details.
    #[serde(rename = "email")]
    pub email: Option<crate::datadogV2::model::TransportWebhookLogEmail>,
    /// The unique email identifier.
    #[serde(rename = "email_id")]
    pub email_id: Option<String>,
    /// The human-readable email type name.
    #[serde(rename = "email_type_display_name")]
    pub email_type_display_name: Option<String>,
    /// The message delivery event details.
    #[serde(rename = "message")]
    pub message: Option<crate::datadogV2::model::TransportWebhookLogMessage>,
    /// The network information for the event.
    #[serde(rename = "network")]
    pub network: Option<crate::datadogV2::model::TransportWebhookLogNetwork>,
    /// The numeric organization identifier.
    #[serde(rename = "org")]
    pub org: Option<i64>,
    /// Metadata about the organization that sent the email.
    #[serde(rename = "org_metadata")]
    pub org_metadata: Option<crate::datadogV2::model::TransportWebhookLogOrgMetadata>,
    /// The organization UUID.
    #[serde(rename = "org_uuid")]
    pub org_uuid: Option<String>,
    /// The timestamp when the email was queued.
    #[serde(rename = "queue_time")]
    pub queue_time: Option<String>,
    /// Indicates whether the open event was triggered by automated machine activity rather than a human recipient (SendGrid-specific).
    #[serde(rename = "sg_machine_open")]
    pub sg_machine_open: Option<bool>,
    /// The email subject line.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    /// The user agent string for open events.
    #[serde(rename = "useragent")]
    pub useragent: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLogAttributes {
    pub fn new() -> TransportWebhookLogAttributes {
        TransportWebhookLogAttributes {
            category: None,
            email: None,
            email_id: None,
            email_type_display_name: None,
            message: None,
            network: None,
            org: None,
            org_metadata: None,
            org_uuid: None,
            queue_time: None,
            sg_machine_open: None,
            subject: None,
            useragent: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(mut self, value: Vec<String>) -> Self {
        self.category = Some(value);
        self
    }

    pub fn email(mut self, value: crate::datadogV2::model::TransportWebhookLogEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn email_id(mut self, value: String) -> Self {
        self.email_id = Some(value);
        self
    }

    pub fn email_type_display_name(mut self, value: String) -> Self {
        self.email_type_display_name = Some(value);
        self
    }

    pub fn message(mut self, value: crate::datadogV2::model::TransportWebhookLogMessage) -> Self {
        self.message = Some(value);
        self
    }

    pub fn network(mut self, value: crate::datadogV2::model::TransportWebhookLogNetwork) -> Self {
        self.network = Some(value);
        self
    }

    pub fn org(mut self, value: i64) -> Self {
        self.org = Some(value);
        self
    }

    pub fn org_metadata(
        mut self,
        value: crate::datadogV2::model::TransportWebhookLogOrgMetadata,
    ) -> Self {
        self.org_metadata = Some(value);
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

    pub fn sg_machine_open(mut self, value: bool) -> Self {
        self.sg_machine_open = Some(value);
        self
    }

    pub fn subject(mut self, value: String) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn useragent(mut self, value: String) -> Self {
        self.useragent = Some(value);
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

impl Default for TransportWebhookLogAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TransportWebhookLogAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogAttributesVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogAttributesVisitor {
            type Value = TransportWebhookLogAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<Vec<String>> = None;
                let mut email: Option<crate::datadogV2::model::TransportWebhookLogEmail> = None;
                let mut email_id: Option<String> = None;
                let mut email_type_display_name: Option<String> = None;
                let mut message: Option<crate::datadogV2::model::TransportWebhookLogMessage> = None;
                let mut network: Option<crate::datadogV2::model::TransportWebhookLogNetwork> = None;
                let mut org: Option<i64> = None;
                let mut org_metadata: Option<
                    crate::datadogV2::model::TransportWebhookLogOrgMetadata,
                > = None;
                let mut org_uuid: Option<String> = None;
                let mut queue_time: Option<String> = None;
                let mut sg_machine_open: Option<bool> = None;
                let mut subject: Option<String> = None;
                let mut useragent: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "network" => {
                            if v.is_null() {
                                continue;
                            }
                            network = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org" => {
                            if v.is_null() {
                                continue;
                            }
                            org = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            org_metadata =
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
                        "sg_machine_open" => {
                            if v.is_null() {
                                continue;
                            }
                            sg_machine_open =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "useragent" => {
                            if v.is_null() {
                                continue;
                            }
                            useragent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TransportWebhookLogAttributes {
                    category,
                    email,
                    email_id,
                    email_type_display_name,
                    message,
                    network,
                    org,
                    org_metadata,
                    org_uuid,
                    queue_time,
                    sg_machine_open,
                    subject,
                    useragent,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogAttributesVisitor)
    }
}
