// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The number of profiled hosts for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageProfilingHour {
    /// Contains the total number of profiled Azure app services reporting during a given hour.
    #[serde(
        rename = "aas_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub aas_count: Option<Option<i64>>,
    /// Get average number of container agents for that hour.
    #[serde(
        rename = "avg_container_agent_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub avg_container_agent_count: Option<Option<i64>>,
    /// Contains the total number of profiled hosts reporting during a given hour.
    #[serde(
        rename = "host_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub host_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<DateTime<Utc>>,
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

impl UsageProfilingHour {
    pub fn new() -> UsageProfilingHour {
        UsageProfilingHour {
            aas_count: None,
            avg_container_agent_count: None,
            host_count: None,
            hour: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn aas_count(mut self, value: Option<i64>) -> Self {
        self.aas_count = Some(value);
        self
    }

    pub fn avg_container_agent_count(mut self, value: Option<i64>) -> Self {
        self.avg_container_agent_count = Some(value);
        self
    }

    pub fn host_count(mut self, value: Option<i64>) -> Self {
        self.host_count = Some(value);
        self
    }

    pub fn hour(mut self, value: DateTime<Utc>) -> Self {
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

impl Default for UsageProfilingHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageProfilingHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageProfilingHourVisitor;
        impl<'a> Visitor<'a> for UsageProfilingHourVisitor {
            type Value = UsageProfilingHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aas_count: Option<Option<i64>> = None;
                let mut avg_container_agent_count: Option<Option<i64>> = None;
                let mut host_count: Option<Option<i64>> = None;
                let mut hour: Option<DateTime<Utc>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aas_count" => {
                            aas_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_container_agent_count" => {
                            avg_container_agent_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host_count" => {
                            host_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = UsageProfilingHour {
                    aas_count,
                    avg_container_agent_count,
                    host_count,
                    hour,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageProfilingHourVisitor)
    }
}
