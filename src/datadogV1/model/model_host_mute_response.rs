// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with the list of muted host for your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMuteResponse {
    /// Action applied to the hosts.
    #[serde(rename = "action")]
    pub action: Option<String>,
    /// POSIX timestamp in seconds when the host is unmuted.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// The host name.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Message associated with the mute.
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMuteResponse {
    pub fn new() -> HostMuteResponse {
        HostMuteResponse {
            action: None,
            end: None,
            hostname: None,
            message: None,
            _unparsed: false,
        }
    }

    pub fn action(mut self, value: String) -> Self {
        self.action = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
}

impl Default for HostMuteResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMuteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMuteResponseVisitor;
        impl<'a> Visitor<'a> for HostMuteResponseVisitor {
            type Value = HostMuteResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut end: Option<i64> = None;
                let mut hostname: Option<String> = None;
                let mut message: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMuteResponse {
                    action,
                    end,
                    hostname,
                    message,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMuteResponseVisitor)
    }
}
