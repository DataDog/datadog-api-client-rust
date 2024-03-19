// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration options for what is shown in an alert event message.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlackIntegrationChannelDisplay {
    /// Show the main body of the alert event.
    #[serde(rename = "message")]
    pub message: Option<bool>,
    /// Show the list of @-handles in the alert event.
    #[serde(rename = "notified")]
    pub notified: Option<bool>,
    /// Show the alert event's snapshot image.
    #[serde(rename = "snapshot")]
    pub snapshot: Option<bool>,
    /// Show the scopes on which the monitor alerted.
    #[serde(rename = "tags")]
    pub tags: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SlackIntegrationChannelDisplay {
    pub fn new() -> SlackIntegrationChannelDisplay {
        SlackIntegrationChannelDisplay {
            message: None,
            notified: None,
            snapshot: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn message(mut self, value: bool) -> Self {
        self.message = Some(value);
        self
    }

    pub fn notified(mut self, value: bool) -> Self {
        self.notified = Some(value);
        self
    }

    pub fn snapshot(mut self, value: bool) -> Self {
        self.snapshot = Some(value);
        self
    }

    pub fn tags(mut self, value: bool) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for SlackIntegrationChannelDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SlackIntegrationChannelDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlackIntegrationChannelDisplayVisitor;
        impl<'a> Visitor<'a> for SlackIntegrationChannelDisplayVisitor {
            type Value = SlackIntegrationChannelDisplay;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<bool> = None;
                let mut notified: Option<bool> = None;
                let mut snapshot: Option<bool> = None;
                let mut tags: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notified" => {
                            if v.is_null() {
                                continue;
                            }
                            notified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SlackIntegrationChannelDisplay {
                    message,
                    notified,
                    snapshot,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SlackIntegrationChannelDisplayVisitor)
    }
}
