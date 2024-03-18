// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object description of an event response attribute.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventResponseAttributes {
    /// Object description of attributes from your event.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::EventAttributes>,
    /// The message of the event.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// An array of tags associated with the event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The timestamp of the event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventResponseAttributes {
    pub fn new() -> EventResponseAttributes {
        EventResponseAttributes {
            attributes: None,
            message: None,
            tags: None,
            timestamp: None,
            _unparsed: false,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::EventAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<Utc>) -> Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for EventResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventResponseAttributesVisitor;
        impl<'a> Visitor<'a> for EventResponseAttributesVisitor {
            type Value = EventResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::EventAttributes> = None;
                let mut message: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<DateTime<Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = EventResponseAttributes {
                    attributes,
                    message,
                    tags,
                    timestamp,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventResponseAttributesVisitor)
    }
}
