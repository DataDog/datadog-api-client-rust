// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An evaluation metric associated with an LLM Observability span.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsSpanEvaluationMetric {
    /// Assessment result (e.g., pass or fail).
    #[serde(rename = "assessment")]
    pub assessment: Option<String>,
    /// Type of the evaluation metric (e.g., score, categorical, boolean).
    #[serde(rename = "eval_metric_type")]
    pub eval_metric_type: Option<String>,
    /// Human-readable reasoning for the evaluation result.
    #[serde(rename = "reasoning")]
    pub reasoning: Option<String>,
    /// Status of the evaluation execution.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Tags associated with the evaluation metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Value of the evaluation result.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsSpanEvaluationMetric {
    pub fn new() -> LLMObsSpanEvaluationMetric {
        LLMObsSpanEvaluationMetric {
            assessment: None,
            eval_metric_type: None,
            reasoning: None,
            status: None,
            tags: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assessment(mut self, value: String) -> Self {
        self.assessment = Some(value);
        self
    }

    pub fn eval_metric_type(mut self, value: String) -> Self {
        self.eval_metric_type = Some(value);
        self
    }

    pub fn reasoning(mut self, value: String) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
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

impl Default for LLMObsSpanEvaluationMetric {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsSpanEvaluationMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsSpanEvaluationMetricVisitor;
        impl<'a> Visitor<'a> for LLMObsSpanEvaluationMetricVisitor {
            type Value = LLMObsSpanEvaluationMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assessment: Option<String> = None;
                let mut eval_metric_type: Option<String> = None;
                let mut reasoning: Option<String> = None;
                let mut status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut value: Option<serde_json::Value> = None;
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
                        }
                        "eval_metric_type" => {
                            if v.is_null() {
                                continue;
                            }
                            eval_metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            if v.is_null() {
                                continue;
                            }
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsSpanEvaluationMetric {
                    assessment,
                    eval_metric_type,
                    reasoning,
                    status,
                    tags,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsSpanEvaluationMetricVisitor)
    }
}
