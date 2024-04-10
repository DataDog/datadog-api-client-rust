// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Downtime match details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorDowntimeMatchResponseAttributes {
    /// The end of the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// An array of groups associated with the downtime.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](<https://docs.datadoghq.com/logs/explorer/search_syntax/>).
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    /// The start of the downtime.
    #[serde(rename = "start")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorDowntimeMatchResponseAttributes {
    pub fn new() -> MonitorDowntimeMatchResponseAttributes {
        MonitorDowntimeMatchResponseAttributes {
            end: None,
            groups: None,
            scope: None,
            start: None,
            _unparsed: false,
        }
    }

    pub fn end(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<String>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn scope(mut self, value: String) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn start(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.start = Some(value);
        self
    }
}

impl Default for MonitorDowntimeMatchResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorDowntimeMatchResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorDowntimeMatchResponseAttributesVisitor;
        impl<'a> Visitor<'a> for MonitorDowntimeMatchResponseAttributesVisitor {
            type Value = MonitorDowntimeMatchResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut scope: Option<String> = None;
                let mut start: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = MonitorDowntimeMatchResponseAttributes {
                    end,
                    groups,
                    scope,
                    start,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorDowntimeMatchResponseAttributesVisitor)
    }
}
