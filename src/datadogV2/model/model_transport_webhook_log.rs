// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single email transport webhook log event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TransportWebhookLog {
    /// Top-level attributes for the webhook log event, including delivery status, recipient details, and provider metadata.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::TransportWebhookLogAttributes,
    /// The ISO 8601 timestamp of the event.
    #[serde(rename = "date")]
    pub date: chrono::DateTime<chrono::Utc>,
    /// The unique log event identifier.
    #[serde(rename = "log_id")]
    pub log_id: String,
    /// The email transport provider.
    #[serde(rename = "source")]
    pub source: String,
    /// The log status level.
    #[serde(rename = "status")]
    pub status: String,
    /// A list of tags associated with the event.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TransportWebhookLog {
    pub fn new(
        attributes: crate::datadogV2::model::TransportWebhookLogAttributes,
        date: chrono::DateTime<chrono::Utc>,
        log_id: String,
        source: String,
        status: String,
        tags: Vec<String>,
    ) -> TransportWebhookLog {
        TransportWebhookLog {
            attributes,
            date,
            log_id,
            source,
            status,
            tags,
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

impl<'de> Deserialize<'de> for TransportWebhookLog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransportWebhookLogVisitor;
        impl<'a> Visitor<'a> for TransportWebhookLogVisitor {
            type Value = TransportWebhookLog;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::TransportWebhookLogAttributes> =
                    None;
                let mut date: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut log_id: Option<String> = None;
                let mut source: Option<String> = None;
                let mut status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "date" => {
                            date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_id" => {
                            log_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let date = date.ok_or_else(|| M::Error::missing_field("date"))?;
                let log_id = log_id.ok_or_else(|| M::Error::missing_field("log_id"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = TransportWebhookLog {
                    attributes,
                    date,
                    log_id,
                    source,
                    status,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TransportWebhookLogVisitor)
    }
}
