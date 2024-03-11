// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single monitor group search result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorGroupSearchResult {
    /// The name of the group.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// The list of tags of the monitor group.
    #[serde(rename = "group_tags")]
    pub group_tags: Option<Vec<String>>,
    /// Latest timestamp the monitor group was in NO_DATA state.
    #[serde(rename = "last_nodata_ts")]
    pub last_nodata_ts: Option<i64>,
    /// Latest timestamp the monitor group triggered.
    #[serde(
        rename = "last_triggered_ts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_triggered_ts: Option<Option<i64>>,
    /// The ID of the monitor.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// The name of the monitor.
    #[serde(rename = "monitor_name")]
    pub monitor_name: Option<String>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorGroupSearchResult {
    pub fn new() -> MonitorGroupSearchResult {
        MonitorGroupSearchResult {
            group: None,
            group_tags: None,
            last_nodata_ts: None,
            last_triggered_ts: None,
            monitor_id: None,
            monitor_name: None,
            status: None,
            _unparsed: false,
        }
    }

    pub fn group(&mut self, value: String) -> &mut Self {
        self.group = Some(value);
        self
    }

    pub fn group_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.group_tags = Some(value);
        self
    }

    pub fn last_nodata_ts(&mut self, value: i64) -> &mut Self {
        self.last_nodata_ts = Some(value);
        self
    }

    pub fn last_triggered_ts(&mut self, value: Option<i64>) -> &mut Self {
        self.last_triggered_ts = Some(value);
        self
    }

    pub fn monitor_id(&mut self, value: i64) -> &mut Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn monitor_name(&mut self, value: String) -> &mut Self {
        self.monitor_name = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV1::model::MonitorOverallStates) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for MonitorGroupSearchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorGroupSearchResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorGroupSearchResultVisitor;
        impl<'a> Visitor<'a> for MonitorGroupSearchResultVisitor {
            type Value = MonitorGroupSearchResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group: Option<String> = None;
                let mut group_tags: Option<Vec<String>> = None;
                let mut last_nodata_ts: Option<i64> = None;
                let mut last_triggered_ts: Option<Option<i64>> = None;
                let mut monitor_id: Option<i64> = None;
                let mut monitor_name: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::MonitorOverallStates> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            group_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_nodata_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_nodata_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_triggered_ts" => {
                            last_triggered_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_name" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::MonitorOverallStates::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = MonitorGroupSearchResult {
                    group,
                    group_tags,
                    last_nodata_ts,
                    last_triggered_ts,
                    monitor_id,
                    monitor_name,
                    status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorGroupSearchResultVisitor)
    }
}
