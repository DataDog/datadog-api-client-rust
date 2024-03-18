// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Online Archive usage in a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageOnlineArchiveHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<DateTime<Utc>>,
    /// Total count of online archived events within the hour.
    #[serde(
        rename = "online_archive_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub online_archive_events_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageOnlineArchiveHour {
    pub fn new() -> UsageOnlineArchiveHour {
        UsageOnlineArchiveHour {
            hour: None,
            online_archive_events_count: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn hour(mut self, value: DateTime<Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn online_archive_events_count(mut self, value: Option<i64>) -> Self {
        self.online_archive_events_count = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageOnlineArchiveHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageOnlineArchiveHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageOnlineArchiveHourVisitor;
        impl<'a> Visitor<'a> for UsageOnlineArchiveHourVisitor {
            type Value = UsageOnlineArchiveHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hour: Option<DateTime<Utc>> = None;
                let mut online_archive_events_count: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "online_archive_events_count" => {
                            online_archive_events_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageOnlineArchiveHour {
                    hour,
                    online_archive_events_count,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageOnlineArchiveHourVisitor)
    }
}
