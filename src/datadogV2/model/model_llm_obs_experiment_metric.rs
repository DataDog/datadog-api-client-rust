// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric associated with an LLM Observability experiment span.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentMetric {
    /// Assessment result for an LLM Observability experiment metric.
    #[serde(rename = "assessment")]
    pub assessment: Option<crate::datadogV2::model::LLMObsMetricAssessment>,
    /// Boolean value. Used when `metric_type` is `boolean`.
    #[serde(rename = "boolean_value")]
    pub boolean_value: Option<bool>,
    /// Categorical value. Used when `metric_type` is `categorical`.
    #[serde(rename = "categorical_value")]
    pub categorical_value: Option<String>,
    /// Error details for an experiment metric evaluation.
    #[serde(rename = "error")]
    pub error: Option<crate::datadogV2::model::LLMObsExperimentMetricError>,
    /// JSON value. Used when `metric_type` is `json`.
    #[serde(rename = "json_value")]
    pub json_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Label or name for the metric.
    #[serde(rename = "label")]
    pub label: String,
    /// Arbitrary metadata associated with the metric.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Type of metric recorded for an LLM Observability experiment.
    #[serde(rename = "metric_type")]
    pub metric_type: crate::datadogV2::model::LLMObsMetricScoreType,
    /// Human-readable reasoning for the metric value.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<String>,
    /// Numeric score value. Used when `metric_type` is `score`.
    #[serde(rename = "score_value")]
    pub score_value: Option<f64>,
    /// Identifier of the span this metric is associated with.
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// List of tags associated with the metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp when the metric was recorded, in milliseconds since Unix epoch.
    #[serde(rename = "timestamp_ms")]
    pub timestamp_ms: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentMetric {
    pub fn new(
        label: String,
        metric_type: crate::datadogV2::model::LLMObsMetricScoreType,
        span_id: String,
        timestamp_ms: i64,
    ) -> LLMObsExperimentMetric {
        LLMObsExperimentMetric {
            assessment: None,
            boolean_value: None,
            categorical_value: None,
            error: None,
            json_value: None,
            label,
            metadata: None,
            metric_type,
            reasoning: None,
            score_value: None,
            span_id,
            tags: None,
            timestamp_ms,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assessment(mut self, value: crate::datadogV2::model::LLMObsMetricAssessment) -> Self {
        self.assessment = Some(value);
        self
    }

    pub fn boolean_value(mut self, value: bool) -> Self {
        self.boolean_value = Some(value);
        self
    }

    pub fn categorical_value(mut self, value: String) -> Self {
        self.categorical_value = Some(value);
        self
    }

    pub fn error(mut self, value: crate::datadogV2::model::LLMObsExperimentMetricError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn json_value(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.json_value = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn reasoning(mut self, value: String) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn score_value(mut self, value: f64) -> Self {
        self.score_value = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentMetricVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentMetricVisitor {
            type Value = LLMObsExperimentMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment: Option<crate::datadogV2::model::LLMObsMetricAssessment> = None;
                let mut boolean_value: Option<bool> = None;
                let mut categorical_value: Option<String> = None;
                let mut error: Option<crate::datadogV2::model::LLMObsExperimentMetricError> = None;
                let mut json_value: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut label: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut metric_type: Option<crate::datadogV2::model::LLMObsMetricScoreType> = None;
                let mut reasoning: Option<String> = None;
                let mut score_value: Option<f64> = None;
                let mut span_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp_ms: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assessment" => {
                            if v.is_null() {
                                continue;
                            }
                            assessment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _assessment) = assessment {
                                match _assessment {
                                    crate::datadogV2::model::LLMObsMetricAssessment::UnparsedObject(_assessment) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "boolean_value" => {
                            if v.is_null() {
                                continue;
                            }
                            boolean_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "categorical_value" => {
                            if v.is_null() {
                                continue;
                            }
                            categorical_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "json_value" => {
                            if v.is_null() {
                                continue;
                            }
                            json_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_type" => {
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric_type) = metric_type {
                                match _metric_type {
                                    crate::datadogV2::model::LLMObsMetricScoreType::UnparsedObject(_metric_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score_value" => {
                            if v.is_null() {
                                continue;
                            }
                            score_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_ms" => {
                            timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let label = label.ok_or_else(|| M::Error::missing_field("label"))?;
                let metric_type =
                    metric_type.ok_or_else(|| M::Error::missing_field("metric_type"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let timestamp_ms =
                    timestamp_ms.ok_or_else(|| M::Error::missing_field("timestamp_ms"))?;

                let content = LLMObsExperimentMetric {
                    assessment,
                    boolean_value,
                    categorical_value,
                    error,
                    json_value,
                    label,
                    metadata,
                    metric_type,
                    reasoning,
                    score_value,
                    span_id,
                    tags,
                    timestamp_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentMetricVisitor)
    }
}
