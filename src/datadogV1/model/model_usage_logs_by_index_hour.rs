// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of indexed logs for each hour and index for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageLogsByIndexHour {
    /// The total number of indexed logs for the queried hour.
    #[serde(rename = "event_count")]
    pub event_count: Option<i64>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
    /// The index ID for this usage.
    #[serde(rename = "index_id")]
    pub index_id: Option<String>,
    /// The user specified name for this index ID.
    #[serde(rename = "index_name")]
    pub index_name: Option<String>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The retention period (in days) for this index ID.
    #[serde(rename = "retention")]
    pub retention: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageLogsByIndexHour {
    pub fn new() -> UsageLogsByIndexHour {
        UsageLogsByIndexHour {
            event_count: None,
            hour: None,
            index_id: None,
            index_name: None,
            org_name: None,
            public_id: None,
            retention: None,
            _unparsed: false,
        }
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn index_id(mut self, value: String) -> Self {
        self.index_id = Some(value);
        self
    }

    pub fn index_name(mut self, value: String) -> Self {
        self.index_name = Some(value);
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

    pub fn retention(mut self, value: i64) -> Self {
        self.retention = Some(value);
        self
    }
}

impl Default for UsageLogsByIndexHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageLogsByIndexHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageLogsByIndexHourVisitor;
        impl<'a> Visitor<'a> for UsageLogsByIndexHourVisitor {
            type Value = UsageLogsByIndexHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut event_count: Option<i64> = None;
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut index_id: Option<String> = None;
                let mut index_name: Option<String> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut retention: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "event_count" => {
                            if v.is_null() {
                                continue;
                            }
                            event_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index_id" => {
                            if v.is_null() {
                                continue;
                            }
                            index_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index_name" => {
                            if v.is_null() {
                                continue;
                            }
                            index_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "retention" => {
                            if v.is_null() {
                                continue;
                            }
                            retention = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageLogsByIndexHour {
                    event_count,
                    hour,
                    index_id,
                    index_name,
                    org_name,
                    public_id,
                    retention,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageLogsByIndexHourVisitor)
    }
}
