// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Global query options that are used during the query.
/// Note: You should only supply timezone or time offset but not both otherwise the query will fail.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansQueryOptions {
    /// The time offset (in seconds) to apply to the query.
    #[serde(rename = "timeOffset")]
    pub time_offset: Option<i64>,
    /// The timezone can be specified as GMT, UTC, an offset from UTC (like UTC+1), or as a Timezone Database identifier (like America/New_York).
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansQueryOptions {
    pub fn new() -> SpansQueryOptions {
        SpansQueryOptions {
            time_offset: None,
            timezone: None,
            _unparsed: false,
        }
    }

    pub fn time_offset(mut self, value: i64) -> Self {
        self.time_offset = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }
}

impl Default for SpansQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansQueryOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansQueryOptionsVisitor;
        impl<'a> Visitor<'a> for SpansQueryOptionsVisitor {
            type Value = SpansQueryOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut time_offset: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "timeOffset" => {
                            if v.is_null() {
                                continue;
                            }
                            time_offset =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SpansQueryOptions {
                    time_offset,
                    timezone,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansQueryOptionsVisitor)
    }
}
