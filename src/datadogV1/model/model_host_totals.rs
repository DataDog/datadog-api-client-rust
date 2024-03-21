// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Total number of host currently monitored by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostTotals {
    /// Total number of active host (UP and ???) reporting to Datadog.
    #[serde(rename = "total_active")]
    pub total_active: Option<i64>,
    /// Number of host that are UP and reporting to Datadog.
    #[serde(rename = "total_up")]
    pub total_up: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostTotals {
    pub fn new() -> HostTotals {
        HostTotals {
            total_active: None,
            total_up: None,
            _unparsed: false,
        }
    }

    pub fn total_active(mut self, value: i64) -> Self {
        self.total_active = Some(value);
        self
    }

    pub fn total_up(mut self, value: i64) -> Self {
        self.total_up = Some(value);
        self
    }
}

impl Default for HostTotals {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HostTotals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostTotalsVisitor;
        impl<'a> Visitor<'a> for HostTotalsVisitor {
            type Value = HostTotals;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total_active: Option<i64> = None;
                let mut total_up: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total_active" => {
                            if v.is_null() {
                                continue;
                            }
                            total_active =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_up" => {
                            if v.is_null() {
                                continue;
                            }
                            total_up = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HostTotals {
                    total_active,
                    total_up,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostTotalsVisitor)
    }
}
