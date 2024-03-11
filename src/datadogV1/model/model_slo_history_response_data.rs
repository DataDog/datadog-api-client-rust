// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An array of service level objective objects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryResponseData {
    /// The `from` timestamp in epoch seconds.
    #[serde(rename = "from_ts")]
    pub from_ts: Option<i64>,
    /// For `metric` based SLOs where the query includes a group-by clause, this represents the list of grouping parameters.
    ///
    /// This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// For grouped SLOs, this represents SLI data for specific groups.
    ///
    /// This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>>,
    /// For multi-monitor SLOs, this represents SLI data for specific monitors.
    ///
    /// This is not included in the responses for `metric` based SLOs.
    #[serde(rename = "monitors")]
    pub monitors: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>>,
    /// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value.
    /// This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
    #[serde(rename = "overall")]
    pub overall: Option<crate::datadogV1::model::SLOHistorySLIData>,
    /// A `metric` based SLO history response.
    ///
    /// This is not included in responses for `monitor` based SLOs.
    #[serde(rename = "series")]
    pub series: Option<crate::datadogV1::model::SLOHistoryMetrics>,
    /// mapping of string timeframe to the SLO threshold.
    #[serde(rename = "thresholds")]
    pub thresholds:
        Option<std::collections::BTreeMap<String, crate::datadogV1::model::SLOThreshold>>,
    /// The `to` timestamp in epoch seconds.
    #[serde(rename = "to_ts")]
    pub to_ts: Option<i64>,
    /// The type of the service level objective.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOType>,
    /// A numeric representation of the type of the service level objective (`0` for
    /// monitor, `1` for metric). Always included in service level objective responses.
    /// Ignored in create/update requests.
    #[serde(rename = "type_id")]
    pub type_id: Option<crate::datadogV1::model::SLOTypeNumeric>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryResponseData {
    pub fn new() -> SLOHistoryResponseData {
        SLOHistoryResponseData {
            from_ts: None,
            group_by: None,
            groups: None,
            monitors: None,
            overall: None,
            series: None,
            thresholds: None,
            to_ts: None,
            type_: None,
            type_id: None,
            _unparsed: false,
        }
    }

    pub fn from_ts(&mut self, value: i64) -> &mut Self {
        self.from_ts = Some(value);
        self
    }

    pub fn group_by(&mut self, value: Vec<String>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn groups(&mut self, value: Vec<crate::datadogV1::model::SLOHistoryMonitor>) -> &mut Self {
        self.groups = Some(value);
        self
    }

    pub fn monitors(
        &mut self,
        value: Vec<crate::datadogV1::model::SLOHistoryMonitor>,
    ) -> &mut Self {
        self.monitors = Some(value);
        self
    }

    pub fn overall(&mut self, value: crate::datadogV1::model::SLOHistorySLIData) -> &mut Self {
        self.overall = Some(value);
        self
    }

    pub fn series(&mut self, value: crate::datadogV1::model::SLOHistoryMetrics) -> &mut Self {
        self.series = Some(value);
        self
    }

    pub fn thresholds(
        &mut self,
        value: std::collections::BTreeMap<String, crate::datadogV1::model::SLOThreshold>,
    ) -> &mut Self {
        self.thresholds = Some(value);
        self
    }

    pub fn to_ts(&mut self, value: i64) -> &mut Self {
        self.to_ts = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV1::model::SLOType) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn type_id(&mut self, value: crate::datadogV1::model::SLOTypeNumeric) -> &mut Self {
        self.type_id = Some(value);
        self
    }
}

impl Default for SLOHistoryResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOHistoryResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryResponseDataVisitor;
        impl<'a> Visitor<'a> for SLOHistoryResponseDataVisitor {
            type Value = SLOHistoryResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_ts: Option<i64> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut groups: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>> = None;
                let mut monitors: Option<Vec<crate::datadogV1::model::SLOHistoryMonitor>> = None;
                let mut overall: Option<crate::datadogV1::model::SLOHistorySLIData> = None;
                let mut series: Option<crate::datadogV1::model::SLOHistoryMetrics> = None;
                let mut thresholds: Option<
                    std::collections::BTreeMap<String, crate::datadogV1::model::SLOThreshold>,
                > = None;
                let mut to_ts: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::SLOType> = None;
                let mut type_id: Option<crate::datadogV1::model::SLOTypeNumeric> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitors" => {
                            if v.is_null() {
                                continue;
                            }
                            monitors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall" => {
                            if v.is_null() {
                                continue;
                            }
                            overall = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "series" => {
                            if v.is_null() {
                                continue;
                            }
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thresholds" => {
                            if v.is_null() {
                                continue;
                            }
                            thresholds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SLOType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type_id" => {
                            if v.is_null() {
                                continue;
                            }
                            type_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_id) = type_id {
                                match _type_id {
                                    crate::datadogV1::model::SLOTypeNumeric::UnparsedObject(
                                        _type_id,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SLOHistoryResponseData {
                    from_ts,
                    group_by,
                    groups,
                    monitors,
                    overall,
                    series,
                    thresholds,
                    to_ts,
                    type_,
                    type_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryResponseDataVisitor)
    }
}
