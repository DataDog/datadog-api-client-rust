// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An evaluation metric event associated with an experiment span.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentEvalMetricEvent {
    /// Assessment result for an LLM Observability experiment metric.
    #[serde(rename = "assessment")]
    pub assessment: Option<crate::datadogV2::model::LLMObsMetricAssessment>,
    /// Boolean value. Present when `metric_type` is `boolean`.
    #[serde(
        rename = "boolean_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub boolean_value: Option<Option<bool>>,
    /// Categorical value. Present when `metric_type` is `categorical`.
    #[serde(
        rename = "categorical_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub categorical_value: Option<Option<String>>,
    /// Source type of the evaluation.
    #[serde(rename = "eval_source_type")]
    pub eval_source_type: Option<String>,
    /// Unique identifier of the evaluation metric event.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// JSON value. Present when `metric_type` is `json`.
    #[serde(
        rename = "json_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub json_value: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Label or name for the metric.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Arbitrary metadata associated with the metric.
    #[serde(
        rename = "metadata",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metadata: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Source of the metric. Either `custom` (user-submitted) or `summary` (experiment-level aggregate).
    #[serde(rename = "metric_source")]
    pub metric_source: Option<String>,
    /// Type of metric recorded for an LLM Observability experiment.
    #[serde(rename = "metric_type")]
    pub metric_type: Option<crate::datadogV2::model::LLMObsMetricScoreType>,
    /// Human-readable reasoning for the metric value.
    #[serde(
        rename = "reasoning",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub reasoning: Option<Option<String>>,
    /// Numeric score. Present when `metric_type` is `score`.
    #[serde(
        rename = "score_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub score_value: Option<Option<f64>>,
    /// Span ID this metric is associated with.
    #[serde(rename = "span_id")]
    pub span_id: Option<String>,
    /// Tags associated with the metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp when the metric was recorded, in milliseconds since Unix epoch.
    #[serde(rename = "timestamp_ms")]
    pub timestamp_ms: Option<i64>,
    /// Trace ID linking this metric to a span.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentEvalMetricEvent {
    pub fn new() -> LLMObsExperimentEvalMetricEvent {
        LLMObsExperimentEvalMetricEvent {
            assessment: None,
            boolean_value: None,
            categorical_value: None,
            eval_source_type: None,
            id: None,
            json_value: None,
            label: None,
            metadata: None,
            metric_source: None,
            metric_type: None,
            reasoning: None,
            score_value: None,
            span_id: None,
            tags: None,
            timestamp_ms: None,
            trace_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assessment(mut self, value: crate::datadogV2::model::LLMObsMetricAssessment) -> Self {
        self.assessment = Some(value);
        self
    }

    pub fn boolean_value(mut self, value: Option<bool>) -> Self {
        self.boolean_value = Some(value);
        self
    }

    pub fn categorical_value(mut self, value: Option<String>) -> Self {
        self.categorical_value = Some(value);
        self
    }

    pub fn eval_source_type(mut self, value: String) -> Self {
        self.eval_source_type = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn json_value(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.json_value = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn metric_source(mut self, value: String) -> Self {
        self.metric_source = Some(value);
        self
    }

    pub fn metric_type(mut self, value: crate::datadogV2::model::LLMObsMetricScoreType) -> Self {
        self.metric_type = Some(value);
        self
    }

    pub fn reasoning(mut self, value: Option<String>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn score_value(mut self, value: Option<f64>) -> Self {
        self.score_value = Some(value);
        self
    }

    pub fn span_id(mut self, value: String) -> Self {
        self.span_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp_ms(mut self, value: i64) -> Self {
        self.timestamp_ms = Some(value);
        self
    }

    pub fn trace_id(mut self, value: String) -> Self {
        self.trace_id = Some(value);
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

impl Default for LLMObsExperimentEvalMetricEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentEvalMetricEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentEvalMetricEventVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentEvalMetricEventVisitor {
            type Value = LLMObsExperimentEvalMetricEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment: Option<crate::datadogV2::model::LLMObsMetricAssessment> = None;
                let mut boolean_value: Option<Option<bool>> = None;
                let mut categorical_value: Option<Option<String>> = None;
                let mut eval_source_type: Option<String> = None;
                let mut id: Option<String> = None;
                let mut json_value: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut label: Option<String> = None;
                let mut metadata: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut metric_source: Option<String> = None;
                let mut metric_type: Option<crate::datadogV2::model::LLMObsMetricScoreType> = None;
                let mut reasoning: Option<Option<String>> = None;
                let mut score_value: Option<Option<f64>> = None;
                let mut span_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp_ms: Option<i64> = None;
                let mut trace_id: Option<String> = None;
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
                            boolean_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "categorical_value" => {
                            categorical_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eval_source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            eval_source_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "json_value" => {
                            json_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_source" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_type" => {
                            if v.is_null() {
                                continue;
                            }
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
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score_value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            score_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            if v.is_null() {
                                continue;
                            }
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsExperimentEvalMetricEvent {
                    assessment,
                    boolean_value,
                    categorical_value,
                    eval_source_type,
                    id,
                    json_value,
                    label,
                    metadata,
                    metric_source,
                    metric_type,
                    reasoning,
                    score_value,
                    span_id,
                    tags,
                    timestamp_ms,
                    trace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentEvalMetricEventVisitor)
    }
}
