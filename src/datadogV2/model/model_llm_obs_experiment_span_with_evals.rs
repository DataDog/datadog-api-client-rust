// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An experiment span enriched with its associated evaluation metrics.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentSpanWithEvals {
    /// ID of the dataset record this span evaluated.
    #[serde(
        rename = "dataset_record_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dataset_record_id: Option<Option<String>>,
    /// Duration of the span in nanoseconds.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Evaluation metrics associated with this span.
    #[serde(rename = "eval_metrics")]
    pub eval_metrics: Option<Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>>,
    /// Unique identifier of the span.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Metadata associated with an experiment span.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::LLMObsExperimentSpanMeta>,
    /// Numeric metrics attached to the span.
    #[serde(rename = "metrics")]
    pub metrics: Option<std::collections::BTreeMap<String, f64>>,
    /// Name of the span.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Parent span ID, if any.
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    /// Span ID.
    #[serde(rename = "span_id")]
    pub span_id: Option<String>,
    /// Start time in nanoseconds since Unix epoch.
    #[serde(rename = "start_ns")]
    pub start_ns: Option<i64>,
    /// Status of the span.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::LLMObsExperimentSpanStatus>,
    /// Tags associated with the span.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Trace ID.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentSpanWithEvals {
    pub fn new() -> LLMObsExperimentSpanWithEvals {
        LLMObsExperimentSpanWithEvals {
            dataset_record_id: None,
            duration: None,
            eval_metrics: None,
            id: None,
            meta: None,
            metrics: None,
            name: None,
            parent_id: None,
            span_id: None,
            start_ns: None,
            status: None,
            tags: None,
            trace_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataset_record_id(mut self, value: Option<String>) -> Self {
        self.dataset_record_id = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn eval_metrics(
        mut self,
        value: Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>,
    ) -> Self {
        self.eval_metrics = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::LLMObsExperimentSpanMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn metrics(mut self, value: std::collections::BTreeMap<String, f64>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn parent_id(mut self, value: String) -> Self {
        self.parent_id = Some(value);
        self
    }

    pub fn span_id(mut self, value: String) -> Self {
        self.span_id = Some(value);
        self
    }

    pub fn start_ns(mut self, value: i64) -> Self {
        self.start_ns = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::LLMObsExperimentSpanStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for LLMObsExperimentSpanWithEvals {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentSpanWithEvals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentSpanWithEvalsVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentSpanWithEvalsVisitor {
            type Value = LLMObsExperimentSpanWithEvals;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_record_id: Option<Option<String>> = None;
                let mut duration: Option<f64> = None;
                let mut eval_metrics: Option<
                    Vec<crate::datadogV2::model::LLMObsExperimentEvalMetricEvent>,
                > = None;
                let mut id: Option<String> = None;
                let mut meta: Option<crate::datadogV2::model::LLMObsExperimentSpanMeta> = None;
                let mut metrics: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut name: Option<String> = None;
                let mut parent_id: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut start_ns: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::LLMObsExperimentSpanStatus> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut trace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_record_id" => {
                            dataset_record_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eval_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            eval_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_id" => {
                            if v.is_null() {
                                continue;
                            }
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            if v.is_null() {
                                continue;
                            }
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_ns" => {
                            if v.is_null() {
                                continue;
                            }
                            start_ns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::LLMObsExperimentSpanStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = LLMObsExperimentSpanWithEvals {
                    dataset_record_id,
                    duration,
                    eval_metrics,
                    id,
                    meta,
                    metrics,
                    name,
                    parent_id,
                    span_id,
                    start_ns,
                    status,
                    tags,
                    trace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentSpanWithEvalsVisitor)
    }
}
