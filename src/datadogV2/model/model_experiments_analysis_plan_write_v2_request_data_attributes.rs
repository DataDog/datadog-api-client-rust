// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Statistical settings and duration targets to apply to the experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
    /// Parameters of the prior distribution to use for Bayesian analysis.
    #[serde(rename = "bayesian_prior")]
    pub bayesian_prior: Option<
        crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataAttributesBayesianPrior,
    >,
    /// Statistical method used to calculate the experiment results.
    #[serde(rename = "confidence_interval_method")]
    pub confidence_interval_method: Option<
        crate::datadogV2::model::ExperimentsAnalysisPlanV2DTODataAttributesConfidenceIntervalMethod,
    >,
    /// Confidence level used for statistical analysis, expressed as a fraction.
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<f64>,
    /// Only a 30-day CUPED lookback is supported.
    #[serde(rename = "cuped_lookback_period_days")]
    pub cuped_lookback_period_days: Option<i64>,
    /// Number of days configured for the experiment to end automatically.
    #[serde(rename = "experiment_auto_end_days")]
    pub experiment_auto_end_days: Option<i64>,
    /// Minimum experiment duration in days configured in the analysis plan.
    #[serde(rename = "experiment_min_duration")]
    pub experiment_min_duration: Option<i64>,
    /// Minimum sample size configured in the analysis plan.
    #[serde(rename = "experiment_min_sample_size")]
    pub experiment_min_sample_size: Option<i64>,
    /// Whether CUPED uses pre-experiment data to reduce variance in the analysis.
    #[serde(rename = "is_cuped_enabled")]
    pub is_cuped_enabled: Option<bool>,
    /// Whether the analysis adjusts for testing multiple hypotheses.
    #[serde(rename = "is_multiple_testing_correction_enabled")]
    pub is_multiple_testing_correction_enabled: Option<bool>,
    /// Weight assigned to the primary metric in the preferential Bonferroni correction.
    #[serde(rename = "preferential_bonferroni_primary_metric_weight")]
    pub preferential_bonferroni_primary_metric_weight: Option<f64>,
    /// Planned experiment duration in days.
    #[serde(
        rename = "target_duration_days",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub target_duration_days: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
    pub fn new() -> ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
        ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
            bayesian_prior: None,
            confidence_interval_method: None,
            confidence_level: None,
            cuped_lookback_period_days: None,
            experiment_auto_end_days: None,
            experiment_min_duration: None,
            experiment_min_sample_size: None,
            is_cuped_enabled: None,
            is_multiple_testing_correction_enabled: None,
            preferential_bonferroni_primary_metric_weight: None,
            target_duration_days: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bayesian_prior(
        mut self,
        value: crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataAttributesBayesianPrior,
    ) -> Self {
        self.bayesian_prior = Some(value);
        self
    }

    pub fn confidence_interval_method(
        mut self,
        value: crate::datadogV2::model::ExperimentsAnalysisPlanV2DTODataAttributesConfidenceIntervalMethod,
    ) -> Self {
        self.confidence_interval_method = Some(value);
        self
    }

    pub fn confidence_level(mut self, value: f64) -> Self {
        self.confidence_level = Some(value);
        self
    }

    pub fn cuped_lookback_period_days(mut self, value: i64) -> Self {
        self.cuped_lookback_period_days = Some(value);
        self
    }

    pub fn experiment_auto_end_days(mut self, value: i64) -> Self {
        self.experiment_auto_end_days = Some(value);
        self
    }

    pub fn experiment_min_duration(mut self, value: i64) -> Self {
        self.experiment_min_duration = Some(value);
        self
    }

    pub fn experiment_min_sample_size(mut self, value: i64) -> Self {
        self.experiment_min_sample_size = Some(value);
        self
    }

    pub fn is_cuped_enabled(mut self, value: bool) -> Self {
        self.is_cuped_enabled = Some(value);
        self
    }

    pub fn is_multiple_testing_correction_enabled(mut self, value: bool) -> Self {
        self.is_multiple_testing_correction_enabled = Some(value);
        self
    }

    pub fn preferential_bonferroni_primary_metric_weight(mut self, value: f64) -> Self {
        self.preferential_bonferroni_primary_metric_weight = Some(value);
        self
    }

    pub fn target_duration_days(mut self, value: Option<i64>) -> Self {
        self.target_duration_days = Some(value);
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

impl Default for ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsAnalysisPlanWriteV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsAnalysisPlanWriteV2RequestDataAttributesVisitor {
            type Value = ExperimentsAnalysisPlanWriteV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bayesian_prior: Option<crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataAttributesBayesianPrior> = None;
                let mut confidence_interval_method: Option<crate::datadogV2::model::ExperimentsAnalysisPlanV2DTODataAttributesConfidenceIntervalMethod> = None;
                let mut confidence_level: Option<f64> = None;
                let mut cuped_lookback_period_days: Option<i64> = None;
                let mut experiment_auto_end_days: Option<i64> = None;
                let mut experiment_min_duration: Option<i64> = None;
                let mut experiment_min_sample_size: Option<i64> = None;
                let mut is_cuped_enabled: Option<bool> = None;
                let mut is_multiple_testing_correction_enabled: Option<bool> = None;
                let mut preferential_bonferroni_primary_metric_weight: Option<f64> = None;
                let mut target_duration_days: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bayesian_prior" => {
                            if v.is_null() {
                                continue;
                            }
                            bayesian_prior =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_interval_method" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_interval_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _confidence_interval_method) =
                                confidence_interval_method
                            {
                                match _confidence_interval_method {
                                    crate::datadogV2::model::ExperimentsAnalysisPlanV2DTODataAttributesConfidenceIntervalMethod::UnparsedObject(_confidence_interval_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "confidence_level" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cuped_lookback_period_days" => {
                            if v.is_null() {
                                continue;
                            }
                            cuped_lookback_period_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_auto_end_days" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_auto_end_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_min_duration" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_min_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_min_sample_size" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_min_sample_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cuped_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_cuped_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_multiple_testing_correction_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_multiple_testing_correction_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "preferential_bonferroni_primary_metric_weight" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            preferential_bonferroni_primary_metric_weight =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_duration_days" => {
                            target_duration_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsAnalysisPlanWriteV2RequestDataAttributes {
                    bayesian_prior,
                    confidence_interval_method,
                    confidence_level,
                    cuped_lookback_period_days,
                    experiment_auto_end_days,
                    experiment_min_duration,
                    experiment_min_sample_size,
                    is_cuped_enabled,
                    is_multiple_testing_correction_enabled,
                    preferential_bonferroni_primary_metric_weight,
                    target_duration_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsAnalysisPlanWriteV2RequestDataAttributesVisitor)
    }
}
