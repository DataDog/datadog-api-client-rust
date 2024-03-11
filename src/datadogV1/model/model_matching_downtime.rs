// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a downtime that matches this monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchingDowntime {
    /// POSIX timestamp to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<i64>>,
    /// The downtime ID.
    #[serde(rename = "id")]
    pub id: i64,
    /// The scope(s) to which the downtime applies. Must be in `key:value` format. For example, `host:app2`.
    /// Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// POSIX timestamp to start the downtime.
    #[serde(rename = "start")]
    pub start: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MatchingDowntime {
    pub fn new(id: i64) -> MatchingDowntime {
        MatchingDowntime {
            end: None,
            id,
            scope: None,
            start: None,
            _unparsed: false,
        }
    }

    pub fn end(&mut self, value: Option<i64>) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn scope(&mut self, value: Vec<String>) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MatchingDowntime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MatchingDowntimeVisitor;
        impl<'a> Visitor<'a> for MatchingDowntimeVisitor {
            type Value = MatchingDowntime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<Option<i64>> = None;
                let mut id: Option<i64> = None;
                let mut scope: Option<Vec<String>> = None;
                let mut start: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = MatchingDowntime {
                    end,
                    id,
                    scope,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MatchingDowntimeVisitor)
    }
}
