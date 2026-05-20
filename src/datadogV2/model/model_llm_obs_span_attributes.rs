// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability span.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsSpanAttributes {
    /// Duration of the span in nanoseconds.
    #[serde(rename = "duration")]
    pub duration: f64,
    /// Evaluation metrics keyed by evaluator name.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::LLMObsSpanEvaluationMetric>,
    >,
    /// Input or output content of an LLM Observability span.
    #[serde(rename = "input")]
    pub input: Option<crate::datadogV2::model::LLMObsSpanIO>,
    /// Detected intent of the span.
    #[serde(rename = "intent")]
    pub intent: Option<String>,
    /// Arbitrary metadata associated with the span.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Numeric metrics associated with the span (e.g., token counts).
    #[serde(rename = "metrics")]
    pub metrics: Option<std::collections::BTreeMap<String, f64>>,
    /// Name of the ML application this span belongs to.
    #[serde(rename = "ml_app")]
    pub ml_app: String,
    /// Name of the model used in this span.
    #[serde(rename = "model_name")]
    pub model_name: Option<String>,
    /// Provider of the model used in this span.
    #[serde(rename = "model_provider")]
    pub model_provider: Option<String>,
    /// Name of the span.
    #[serde(rename = "name")]
    pub name: String,
    /// Input or output content of an LLM Observability span.
    #[serde(rename = "output")]
    pub output: Option<crate::datadogV2::model::LLMObsSpanIO>,
    /// Identifier of the parent span, if any.
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    /// Unique identifier of the span.
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// Kind of span (e.g., llm, agent, tool, task, workflow).
    #[serde(rename = "span_kind")]
    pub span_kind: String,
    /// Start time of the span in nanoseconds since Unix epoch.
    #[serde(rename = "start_ns")]
    pub start_ns: i64,
    /// Status of the span (e.g., ok, error).
    #[serde(rename = "status")]
    pub status: String,
    /// Tags associated with the span.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Tool definitions available to the span.
    #[serde(rename = "tool_definitions")]
    pub tool_definitions: Option<Vec<crate::datadogV2::model::LLMObsSpanToolDefinition>>,
    /// Trace identifier this span belongs to.
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsSpanAttributes {
    pub fn new(
        duration: f64,
        ml_app: String,
        name: String,
        span_id: String,
        span_kind: String,
        start_ns: i64,
        status: String,
        trace_id: String,
    ) -> LLMObsSpanAttributes {
        LLMObsSpanAttributes {
            duration,
            evaluation: None,
            input: None,
            intent: None,
            metadata: None,
            metrics: None,
            ml_app,
            model_name: None,
            model_provider: None,
            name,
            output: None,
            parent_id: None,
            span_id,
            span_kind,
            start_ns,
            status,
            tags: None,
            tool_definitions: None,
            trace_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn evaluation(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::LLMObsSpanEvaluationMetric,
        >,
    ) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn input(mut self, value: crate::datadogV2::model::LLMObsSpanIO) -> Self {
        self.input = Some(value);
        self
    }

    pub fn intent(mut self, value: String) -> Self {
        self.intent = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn metrics(mut self, value: std::collections::BTreeMap<String, f64>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn model_name(mut self, value: String) -> Self {
        self.model_name = Some(value);
        self
    }

    pub fn model_provider(mut self, value: String) -> Self {
        self.model_provider = Some(value);
        self
    }

    pub fn output(mut self, value: crate::datadogV2::model::LLMObsSpanIO) -> Self {
        self.output = Some(value);
        self
    }

    pub fn parent_id(mut self, value: String) -> Self {
        self.parent_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn tool_definitions(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsSpanToolDefinition>,
    ) -> Self {
        self.tool_definitions = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsSpanAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsSpanAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsSpanAttributesVisitor {
            type Value = LLMObsSpanAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<f64> = None;
                let mut evaluation: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::LLMObsSpanEvaluationMetric,
                    >,
                > = None;
                let mut input: Option<crate::datadogV2::model::LLMObsSpanIO> = None;
                let mut intent: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut metrics: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut ml_app: Option<String> = None;
                let mut model_name: Option<String> = None;
                let mut model_provider: Option<String> = None;
                let mut name: Option<String> = None;
                let mut output: Option<crate::datadogV2::model::LLMObsSpanIO> = None;
                let mut parent_id: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut span_kind: Option<String> = None;
                let mut start_ns: Option<i64> = None;
                let mut status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut tool_definitions: Option<
                    Vec<crate::datadogV2::model::LLMObsSpanToolDefinition>,
                > = None;
                let mut trace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluation" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input" => {
                            if v.is_null() {
                                continue;
                            }
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "intent" => {
                            if v.is_null() {
                                continue;
                            }
                            intent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_app" => {
                            ml_app = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_name" => {
                            if v.is_null() {
                                continue;
                            }
                            model_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "model_provider" => {
                            if v.is_null() {
                                continue;
                            }
                            model_provider =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output" => {
                            if v.is_null() {
                                continue;
                            }
                            output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_id" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_kind" => {
                            span_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_ns" => {
                            start_ns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_definitions" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_definitions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_id" => {
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;
                let ml_app = ml_app.ok_or_else(|| M::Error::missing_field("ml_app"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let span_kind = span_kind.ok_or_else(|| M::Error::missing_field("span_kind"))?;
                let start_ns = start_ns.ok_or_else(|| M::Error::missing_field("start_ns"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let trace_id = trace_id.ok_or_else(|| M::Error::missing_field("trace_id"))?;

                let content = LLMObsSpanAttributes {
                    duration,
                    evaluation,
                    input,
                    intent,
                    metadata,
                    metrics,
                    ml_app,
                    model_name,
                    model_provider,
                    name,
                    output,
                    parent_id,
                    span_id,
                    span_kind,
                    start_ns,
                    status,
                    tags,
                    tool_definitions,
                    trace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsSpanAttributesVisitor)
    }
}
