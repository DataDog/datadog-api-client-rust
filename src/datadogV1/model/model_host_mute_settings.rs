// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Combination of settings to mute a host.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMuteSettings {
    /// POSIX timestamp in seconds when the host is unmuted. If omitted, the host remains muted until explicitly unmuted.
    #[serde(rename = "end")]
    pub end: Option<i64>,
    /// Message to associate with the muting of this host.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// If true and the host is already muted, replaces existing host mute settings.
    #[serde(rename = "override")]
    pub override_: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMuteSettings {
    pub fn new() -> HostMuteSettings {
        HostMuteSettings {
            end: None,
            message: None,
            override_: None,
            _unparsed: false,
        }
    }

    pub fn end(&mut self, value: i64) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn override_(&mut self, value: bool) -> &mut Self {
        self.override_ = Some(value);
        self
    }
}

impl Default for HostMuteSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostMuteSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMuteSettingsVisitor;
        impl<'a> Visitor<'a> for HostMuteSettingsVisitor {
            type Value = HostMuteSettings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut override_: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "override" => {
                            if v.is_null() {
                                continue;
                            }
                            override_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostMuteSettings {
                    end,
                    message,
                    override_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostMuteSettingsVisitor)
    }
}
