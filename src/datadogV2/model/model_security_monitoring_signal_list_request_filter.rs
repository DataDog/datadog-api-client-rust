// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Search filters for listing security signals.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalListRequestFilter {
    /// The minimum timestamp for requested security signals.
    #[serde(rename = "from")]
    pub from: Option<DateTime<Utc>>,
    /// Search query for listing security signals.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum timestamp for requested security signals.
    #[serde(rename = "to")]
    pub to: Option<DateTime<Utc>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalListRequestFilter {
    pub fn new() -> SecurityMonitoringSignalListRequestFilter {
        SecurityMonitoringSignalListRequestFilter {
            from: None,
            query: None,
            to: None,
            _unparsed: false,
        }
    }

    pub fn from(mut self, value: DateTime<Utc>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<Utc>) -> Self {
        self.to = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignalListRequestFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalListRequestFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalListRequestFilterVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalListRequestFilterVisitor {
            type Value = SecurityMonitoringSignalListRequestFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<DateTime<Utc>> = None;
                let mut query: Option<String> = None;
                let mut to: Option<DateTime<Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringSignalListRequestFilter {
                    from,
                    query,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalListRequestFilterVisitor)
    }
}
