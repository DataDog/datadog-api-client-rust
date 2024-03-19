// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Database Monitoring usage for a given organization for a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageDBMHour {
    /// The total number of Database Monitoring host hours from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "dbm_host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dbm_host_count: Option<Option<i64>>,
    /// The total number of normalized Database Monitoring queries from the start of the given hour’s month until the given hour.
    #[serde(
        rename = "dbm_queries_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dbm_queries_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<chrono::DateTime<chrono::Utc>>,
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

impl UsageDBMHour {
    pub fn new() -> UsageDBMHour {
        UsageDBMHour {
            dbm_host_count: None,
            dbm_queries_count: None,
            hour: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn dbm_host_count(mut self, value: Option<i64>) -> Self {
        self.dbm_host_count = Some(value);
        self
    }

    pub fn dbm_queries_count(mut self, value: Option<i64>) -> Self {
        self.dbm_queries_count = Some(value);
        self
    }

    pub fn hour(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.hour = Some(value);
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

impl Default for UsageDBMHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageDBMHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageDBMHourVisitor;
        impl<'a> Visitor<'a> for UsageDBMHourVisitor {
            type Value = UsageDBMHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dbm_host_count: Option<Option<i64>> = None;
                let mut dbm_queries_count: Option<Option<i64>> = None;
                let mut hour: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dbm_host_count" => {
                            dbm_host_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dbm_queries_count" => {
                            dbm_queries_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hour" => {
                            if v.is_null() {
                                continue;
                            }
                            hour = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = UsageDBMHour {
                    dbm_host_count,
                    dbm_queries_count,
                    hour,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageDBMHourVisitor)
    }
}
