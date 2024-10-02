// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the uptime information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsUptime {
    /// An array of error objects returned while querying the history data for the service level objective.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option")]
    pub errors: Option<Option<Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>>>,
    /// The location name
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// The state transition history for the monitor, represented as an array of
    /// pairs. Each pair is an array where the first element is the transition timestamp
    /// in Unix epoch format (integer) and the second element is the state (integer).
    /// For the state, an integer value of `0` indicates uptime, `1` indicates downtime,
    /// and `2` indicates no data.
    #[serde(rename = "history")]
    pub history: Option<Vec<Vec<f64>>>,
    /// The number of decimal places to which the SLI value is accurate for the given from-to timestamps.
    #[serde(rename = "span_precision")]
    pub span_precision: Option<f64>,
    /// The overall uptime.
    #[serde(rename = "uptime")]
    pub uptime: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsUptime {
    pub fn new() -> SyntheticsUptime {
        SyntheticsUptime {
            errors: None,
            group: None,
            history: None,
            span_precision: None,
            uptime: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn errors(
        mut self,
        value: Option<Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>>,
    ) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn group(mut self, value: String) -> Self {
        self.group = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<Vec<f64>>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn span_precision(mut self, value: f64) -> Self {
        self.span_precision = Some(value);
        self
    }

    pub fn uptime(mut self, value: f64) -> Self {
        self.uptime = Some(value);
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

impl Default for SyntheticsUptime {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsUptime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsUptimeVisitor;
        impl<'a> Visitor<'a> for SyntheticsUptimeVisitor {
            type Value = SyntheticsUptime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut errors: Option<
                    Option<Vec<crate::datadogV1::model::SLOHistoryResponseErrorWithType>>,
                > = None;
                let mut group: Option<String> = None;
                let mut history: Option<Vec<Vec<f64>>> = None;
                let mut span_precision: Option<f64> = None;
                let mut uptime: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "errors" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsUptime {
                    errors,
                    group,
                    history,
                    span_precision,
                    uptime,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsUptimeVisitor)
    }
}
