// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The number of indexed logs for each hour for a given organization broken down by retention period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageLogsByRetentionHour {
    /// Total logs indexed with this retention period during a given hour.
    #[serde(
        rename = "indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub indexed_events_count: Option<Option<i64>>,
    /// Live logs indexed with this retention period during a given hour.
    #[serde(
        rename = "live_indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub live_indexed_events_count: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Rehydrated logs indexed with this retention period during a given hour.
    #[serde(
        rename = "rehydrated_indexed_events_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rehydrated_indexed_events_count: Option<Option<i64>>,
    /// The retention period in days or "custom" for all custom retention usage.
    #[serde(
        rename = "retention",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub retention: Option<Option<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageLogsByRetentionHour {
    pub fn new() -> UsageLogsByRetentionHour {
        UsageLogsByRetentionHour {
            indexed_events_count: None,
            live_indexed_events_count: None,
            org_name: None,
            public_id: None,
            rehydrated_indexed_events_count: None,
            retention: None,
            _unparsed: false,
        }
    }

    pub fn indexed_events_count(mut self, value: Option<i64>) -> Self {
        self.indexed_events_count = Some(value);
        self
    }

    pub fn live_indexed_events_count(mut self, value: Option<i64>) -> Self {
        self.live_indexed_events_count = Some(value);
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

    pub fn rehydrated_indexed_events_count(mut self, value: Option<i64>) -> Self {
        self.rehydrated_indexed_events_count = Some(value);
        self
    }

    pub fn retention(mut self, value: Option<String>) -> Self {
        self.retention = Some(value);
        self
    }
}

impl Default for UsageLogsByRetentionHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageLogsByRetentionHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageLogsByRetentionHourVisitor;
        impl<'a> Visitor<'a> for UsageLogsByRetentionHourVisitor {
            type Value = UsageLogsByRetentionHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut indexed_events_count: Option<Option<i64>> = None;
                let mut live_indexed_events_count: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut rehydrated_indexed_events_count: Option<Option<i64>> = None;
                let mut retention: Option<Option<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "indexed_events_count" => {
                            indexed_events_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "live_indexed_events_count" => {
                            live_indexed_events_count =
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
                        "rehydrated_indexed_events_count" => {
                            rehydrated_indexed_events_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention" => {
                            retention = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageLogsByRetentionHour {
                    indexed_events_count,
                    live_indexed_events_count,
                    org_name,
                    public_id,
                    rehydrated_indexed_events_count,
                    retention,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageLogsByRetentionHourVisitor)
    }
}
