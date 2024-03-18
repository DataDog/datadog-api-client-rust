// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of the last API test run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPITestResultShortResult {
    /// Describes if the test run has passed or failed.
    #[serde(rename = "passed")]
    pub passed: Option<bool>,
    /// Object containing all metrics and their values collected for a Synthetic API test.
    /// See the [Synthetic Monitoring Metrics documentation](<https://docs.datadoghq.com/synthetics/metrics/>).
    #[serde(rename = "timings")]
    pub timings: Option<crate::datadogV1::model::SyntheticsTiming>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPITestResultShortResult {
    pub fn new() -> SyntheticsAPITestResultShortResult {
        SyntheticsAPITestResultShortResult {
            passed: None,
            timings: None,
            _unparsed: false,
        }
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn timings(mut self, value: crate::datadogV1::model::SyntheticsTiming) -> Self {
        self.timings = Some(value);
        self
    }
}

impl Default for SyntheticsAPITestResultShortResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsAPITestResultShortResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestResultShortResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestResultShortResultVisitor {
            type Value = SyntheticsAPITestResultShortResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut passed: Option<bool> = None;
                let mut timings: Option<crate::datadogV1::model::SyntheticsTiming> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "passed" => {
                            if v.is_null() {
                                continue;
                            }
                            passed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsAPITestResultShortResult {
                    passed,
                    timings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestResultShortResultVisitor)
    }
}
