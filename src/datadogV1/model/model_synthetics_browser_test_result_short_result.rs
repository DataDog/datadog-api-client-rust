// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with the result of the last browser test run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestResultShortResult {
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDevice>,
    /// Length in milliseconds of the browser test run.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Amount of errors collected for a single browser test run.
    #[serde(rename = "errorCount")]
    pub error_count: Option<i64>,
    /// Amount of browser test steps completed before failing.
    #[serde(rename = "stepCountCompleted")]
    pub step_count_completed: Option<i64>,
    /// Total amount of browser test steps.
    #[serde(rename = "stepCountTotal")]
    pub step_count_total: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestResultShortResult {
    pub fn new() -> SyntheticsBrowserTestResultShortResult {
        SyntheticsBrowserTestResultShortResult {
            device: None,
            duration: None,
            error_count: None,
            step_count_completed: None,
            step_count_total: None,
            _unparsed: false,
        }
    }

    pub fn device(&mut self, value: crate::datadogV1::model::SyntheticsDevice) -> &mut Self {
        self.device = Some(value);
        self
    }

    pub fn duration(&mut self, value: f64) -> &mut Self {
        self.duration = Some(value);
        self
    }

    pub fn error_count(&mut self, value: i64) -> &mut Self {
        self.error_count = Some(value);
        self
    }

    pub fn step_count_completed(&mut self, value: i64) -> &mut Self {
        self.step_count_completed = Some(value);
        self
    }

    pub fn step_count_total(&mut self, value: i64) -> &mut Self {
        self.step_count_total = Some(value);
        self
    }
}

impl Default for SyntheticsBrowserTestResultShortResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTestResultShortResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestResultShortResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestResultShortResultVisitor {
            type Value = SyntheticsBrowserTestResultShortResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut device: Option<crate::datadogV1::model::SyntheticsDevice> = None;
                let mut duration: Option<f64> = None;
                let mut error_count: Option<i64> = None;
                let mut step_count_completed: Option<i64> = None;
                let mut step_count_total: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errorCount" => {
                            if v.is_null() {
                                continue;
                            }
                            error_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stepCountCompleted" => {
                            if v.is_null() {
                                continue;
                            }
                            step_count_completed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stepCountTotal" => {
                            if v.is_null() {
                                continue;
                            }
                            step_count_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsBrowserTestResultShortResult {
                    device,
                    duration,
                    error_count,
                    step_count_completed,
                    step_count_total,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestResultShortResultVisitor)
    }
}
