// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with the results of a single Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestResultShort {
    /// Last time the browser test was performed.
    #[serde(rename = "check_time")]
    pub check_time: Option<f64>,
    /// Location from which the Browser test was performed.
    #[serde(rename = "probe_dc")]
    pub probe_dc: Option<String>,
    /// Object with the result of the last browser test run.
    #[serde(rename = "result")]
    pub result: Option<crate::datadogV1::model::SyntheticsBrowserTestResultShortResult>,
    /// ID of the browser test result.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// The status of your Synthetic monitor.
    /// * `O` for not triggered
    /// * `1` for triggered
    /// * `2` for no data
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestMonitorStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestResultShort {
    pub fn new() -> SyntheticsBrowserTestResultShort {
        SyntheticsBrowserTestResultShort {
            check_time: None,
            probe_dc: None,
            result: None,
            result_id: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn check_time(mut self, value: f64) -> Self {
        self.check_time = Some(value);
        self
    }

    pub fn probe_dc(mut self, value: String) -> Self {
        self.probe_dc = Some(value);
        self
    }

    pub fn result(
        mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestResultShortResult,
    ) -> Self {
        self.result = Some(value);
        self
    }

    pub fn result_id(mut self, value: String) -> Self {
        self.result_id = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::SyntheticsTestMonitorStatus) -> Self {
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

impl Default for SyntheticsBrowserTestResultShort {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTestResultShort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestResultShortVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestResultShortVisitor {
            type Value = SyntheticsBrowserTestResultShort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut check_time: Option<f64> = None;
                let mut probe_dc: Option<String> = None;
                let mut result: Option<
                    crate::datadogV1::model::SyntheticsBrowserTestResultShortResult,
                > = None;
                let mut result_id: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsTestMonitorStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "check_time" => {
                            if v.is_null() {
                                continue;
                            }
                            check_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "probe_dc" => {
                            if v.is_null() {
                                continue;
                            }
                            probe_dc = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result" => {
                            if v.is_null() {
                                continue;
                            }
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_id" => {
                            if v.is_null() {
                                continue;
                            }
                            result_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::SyntheticsTestMonitorStatus::UnparsedObject(_status) => {
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

                let content = SyntheticsBrowserTestResultShort {
                    check_time,
                    probe_dc,
                    result,
                    result_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestResultShortVisitor)
    }
}
