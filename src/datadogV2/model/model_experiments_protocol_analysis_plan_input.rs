// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Statistical settings. Use `default_duration_days` for duration; `target_duration_days` is not accepted here.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsProtocolAnalysisPlanInput {
    /// Whether to use pre-experiment data to reduce variance with CUPED.
    #[serde(
        rename = "compute_cuped",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub compute_cuped: Option<Option<bool>>,
    /// Confidence interval method: `Sequential`, `FixedSample`, `Bayesian`, or `SequentialFixedHybrid`. An empty string leaves the method unset.
    #[serde(rename = "confidence_interval_method")]
    pub confidence_interval_method: Option<String>,
    /// Confidence level used by the statistical analysis.
    #[serde(
        rename = "confidence_level",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub confidence_level: Option<Option<f64>>,
    /// Method used to adjust for testing multiple metrics.
    #[serde(
        rename = "multiple_testing_correction_method",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub multiple_testing_correction_method: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsProtocolAnalysisPlanInput {
    pub fn new() -> ExperimentsProtocolAnalysisPlanInput {
        ExperimentsProtocolAnalysisPlanInput {
            compute_cuped: None,
            confidence_interval_method: None,
            confidence_level: None,
            multiple_testing_correction_method: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compute_cuped(mut self, value: Option<bool>) -> Self {
        self.compute_cuped = Some(value);
        self
    }

    pub fn confidence_interval_method(mut self, value: String) -> Self {
        self.confidence_interval_method = Some(value);
        self
    }

    pub fn confidence_level(mut self, value: Option<f64>) -> Self {
        self.confidence_level = Some(value);
        self
    }

    pub fn multiple_testing_correction_method(mut self, value: Option<String>) -> Self {
        self.multiple_testing_correction_method = Some(value);
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

impl Default for ExperimentsProtocolAnalysisPlanInput {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsProtocolAnalysisPlanInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsProtocolAnalysisPlanInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsProtocolAnalysisPlanInputVisitor {
            type Value = ExperimentsProtocolAnalysisPlanInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute_cuped: Option<Option<bool>> = None;
                let mut confidence_interval_method: Option<String> = None;
                let mut confidence_level: Option<Option<f64>> = None;
                let mut multiple_testing_correction_method: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute_cuped" => {
                            compute_cuped =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_interval_method" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_interval_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_level" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "multiple_testing_correction_method" => {
                            multiple_testing_correction_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsProtocolAnalysisPlanInput {
                    compute_cuped,
                    confidence_interval_method,
                    confidence_level,
                    multiple_testing_correction_method,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsProtocolAnalysisPlanInputVisitor)
    }
}
