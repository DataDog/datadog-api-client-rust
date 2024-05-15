// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value.
/// This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOHistoryMonitor {
    /// A mapping of threshold `timeframe` to the remaining error budget.
    #[serde(rename = "error_budget_remaining")]
    pub error_budget_remaining: Option<std::collections::BTreeMap<String, f64>>,
    /// An array of error objects returned while querying the history data for the service level objective.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>>,
    /// For groups in a grouped SLO, this is the group name.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// The state transition history for the monitor. It is represented as
    /// an array of pairs. Each pair is an array containing the timestamp of the transition
    /// as an integer in Unix epoch format in the first element, and the state as an integer in the
    /// second element. An integer value of `0` for state means uptime, `1` means downtime, and `2` means no data.
    /// Periods of no data are counted either as uptime or downtime depending on monitor settings.
    /// See [SLO documentatio](<https://docs.datadoghq.com/service_management/service_level_objectives/monitor/#missing-data>)
    /// for detailed information.
    #[serde(rename = "history")]
    pub history: Option<Vec<Vec<f64>>>,
    /// For `monitor` based SLOs, this is the last modified timestamp in epoch seconds of the monitor.
    #[serde(rename = "monitor_modified")]
    pub monitor_modified: Option<i64>,
    /// For `monitor` based SLOs, this describes the type of monitor.
    #[serde(rename = "monitor_type")]
    pub monitor_type: Option<String>,
    /// For groups in a grouped SLO, this is the group name. For monitors in a multi-monitor SLO, this is the monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp. Use `span_precision` instead.
    #[deprecated]
    #[serde(rename = "precision")]
    pub precision: Option<f64>,
    /// For `monitor` based SLOs, when `true` this indicates that a replay is in progress to give an accurate uptime
    /// calculation.
    #[serde(rename = "preview")]
    pub preview: Option<bool>,
    /// The current SLI value of the SLO over the history window.
    #[serde(
        rename = "sli_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub sli_value: Option<Option<f64>>,
    /// The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp.
    #[serde(rename = "span_precision")]
    pub span_precision: Option<f64>,
    /// Use `sli_value` instead.
    #[deprecated]
    #[serde(rename = "uptime")]
    pub uptime: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOHistoryMonitor {
    pub fn new() -> SLOHistoryMonitor {
        #[allow(deprecated)]
        SLOHistoryMonitor {
            error_budget_remaining: None,
            errors: None,
            group: None,
            history: None,
            monitor_modified: None,
            monitor_type: None,
            name: None,
            precision: None,
            preview: None,
            sli_value: None,
            span_precision: None,
            uptime: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn error_budget_remaining(
        mut self,
        value: std::collections::BTreeMap<String, f64>,
    ) -> Self {
        self.error_budget_remaining = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn errors(
        mut self,
        value: Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>,
    ) -> Self {
        self.errors = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group(mut self, value: String) -> Self {
        self.group = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn history(mut self, value: Vec<Vec<f64>>) -> Self {
        self.history = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitor_modified(mut self, value: i64) -> Self {
        self.monitor_modified = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn monitor_type(mut self, value: String) -> Self {
        self.monitor_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn precision(mut self, value: f64) -> Self {
        self.precision = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn preview(mut self, value: bool) -> Self {
        self.preview = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sli_value(mut self, value: Option<f64>) -> Self {
        self.sli_value = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn span_precision(mut self, value: f64) -> Self {
        self.span_precision = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn uptime(mut self, value: f64) -> Self {
        self.uptime = Some(value);
        self
    }
}

impl Default for SLOHistoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOHistoryMonitor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOHistoryMonitorVisitor;
        impl<'a> Visitor<'a> for SLOHistoryMonitorVisitor {
            type Value = SLOHistoryMonitor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_budget_remaining: Option<std::collections::BTreeMap<String, f64>> =
                    None;
                let mut errors: Option<
                    Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>,
                > = None;
                let mut group: Option<String> = None;
                let mut history: Option<Vec<Vec<f64>>> = None;
                let mut monitor_modified: Option<i64> = None;
                let mut monitor_type: Option<String> = None;
                let mut name: Option<String> = None;
                let mut precision: Option<f64> = None;
                let mut preview: Option<bool> = None;
                let mut sli_value: Option<Option<f64>> = None;
                let mut span_precision: Option<f64> = None;
                let mut uptime: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_budget_remaining" => {
                            if v.is_null() {
                                continue;
                            }
                            error_budget_remaining =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "history" => {
                            if v.is_null() {
                                continue;
                            }
                            history = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_modified" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_modified =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_type" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "precision" => {
                            if v.is_null() {
                                continue;
                            }
                            precision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preview" => {
                            if v.is_null() {
                                continue;
                            }
                            preview = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sli_value" => {
                            sli_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_precision" => {
                            if v.is_null() {
                                continue;
                            }
                            span_precision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uptime" => {
                            if v.is_null() {
                                continue;
                            }
                            uptime = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                #[allow(deprecated)]
                let content = SLOHistoryMonitor {
                    error_budget_remaining,
                    errors,
                    group,
                    history,
                    monitor_modified,
                    monitor_type,
                    name,
                    precision,
                    preview,
                    sli_value,
                    span_precision,
                    uptime,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOHistoryMonitorVisitor)
    }
}
