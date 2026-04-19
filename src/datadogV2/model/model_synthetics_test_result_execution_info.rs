// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Execution details for a Synthetic test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultExecutionInfo {
    /// Total duration of the test execution in milliseconds.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Error message if the execution encountered an issue.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Whether this result is from a fast retry.
    #[serde(rename = "is_fast_retry")]
    pub is_fast_retry: Option<bool>,
    /// Timing breakdown of the test execution in milliseconds.
    #[serde(rename = "timings")]
    pub timings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether the test was executed through a tunnel.
    #[serde(rename = "tunnel")]
    pub tunnel: Option<bool>,
    /// Whether the location was unhealthy during execution.
    #[serde(rename = "unhealthy")]
    pub unhealthy: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultExecutionInfo {
    pub fn new() -> SyntheticsTestResultExecutionInfo {
        SyntheticsTestResultExecutionInfo {
            duration: None,
            error_message: None,
            is_fast_retry: None,
            timings: None,
            tunnel: None,
            unhealthy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn is_fast_retry(mut self, value: bool) -> Self {
        self.is_fast_retry = Some(value);
        self
    }

    pub fn timings(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.timings = Some(value);
        self
    }

    pub fn tunnel(mut self, value: bool) -> Self {
        self.tunnel = Some(value);
        self
    }

    pub fn unhealthy(mut self, value: bool) -> Self {
        self.unhealthy = Some(value);
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

impl Default for SyntheticsTestResultExecutionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultExecutionInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultExecutionInfoVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultExecutionInfoVisitor {
            type Value = SyntheticsTestResultExecutionInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<f64> = None;
                let mut error_message: Option<String> = None;
                let mut is_fast_retry: Option<bool> = None;
                let mut timings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut tunnel: Option<bool> = None;
                let mut unhealthy: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fast_retry" => {
                            if v.is_null() {
                                continue;
                            }
                            is_fast_retry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tunnel" => {
                            if v.is_null() {
                                continue;
                            }
                            tunnel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unhealthy" => {
                            if v.is_null() {
                                continue;
                            }
                            unhealthy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultExecutionInfo {
                    duration,
                    error_message,
                    is_fast_retry,
                    timings,
                    tunnel,
                    unhealthy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultExecutionInfoVisitor)
    }
}
