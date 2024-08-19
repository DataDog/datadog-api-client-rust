// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Monitor state for a single group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorStateGroup {
    /// Latest timestamp the monitor was in NO_DATA state.
    #[serde(rename = "last_nodata_ts")]
    pub last_nodata_ts: Option<i64>,
    /// Latest timestamp of the notification sent for this monitor group.
    #[serde(rename = "last_notified_ts")]
    pub last_notified_ts: Option<i64>,
    /// Latest timestamp the monitor group was resolved.
    #[serde(rename = "last_resolved_ts")]
    pub last_resolved_ts: Option<i64>,
    /// Latest timestamp the monitor group triggered.
    #[serde(rename = "last_triggered_ts")]
    pub last_triggered_ts: Option<i64>,
    /// The name of the monitor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorStateGroup {
    pub fn new() -> MonitorStateGroup {
        MonitorStateGroup {
            last_nodata_ts: None,
            last_notified_ts: None,
            last_resolved_ts: None,
            last_triggered_ts: None,
            name: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_nodata_ts(mut self, value: i64) -> Self {
        self.last_nodata_ts = Some(value);
        self
    }

    pub fn last_notified_ts(mut self, value: i64) -> Self {
        self.last_notified_ts = Some(value);
        self
    }

    pub fn last_resolved_ts(mut self, value: i64) -> Self {
        self.last_resolved_ts = Some(value);
        self
    }

    pub fn last_triggered_ts(mut self, value: i64) -> Self {
        self.last_triggered_ts = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::MonitorOverallStates) -> Self {
        self.status = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for MonitorStateGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorStateGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorStateGroupVisitor;
        impl<'a> Visitor<'a> for MonitorStateGroupVisitor {
            type Value = MonitorStateGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_nodata_ts: Option<i64> = None;
                let mut last_notified_ts: Option<i64> = None;
                let mut last_resolved_ts: Option<i64> = None;
                let mut last_triggered_ts: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::MonitorOverallStates> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_nodata_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_nodata_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_notified_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_notified_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_resolved_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_resolved_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_triggered_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_triggered_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorStateGroup {
                    last_nodata_ts,
                    last_notified_ts,
                    last_resolved_ts,
                    last_triggered_ts,
                    name,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorStateGroupVisitor)
    }
}
