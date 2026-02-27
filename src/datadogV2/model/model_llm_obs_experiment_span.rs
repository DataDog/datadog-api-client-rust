// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A span associated with an LLM Observability experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentSpan {
    /// Dataset ID associated with this span.
    #[serde(rename = "dataset_id")]
    pub dataset_id: String,
    /// Duration of the span in nanoseconds.
    #[serde(rename = "duration")]
    pub duration: i64,
    /// Metadata associated with an experiment span.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::LLMObsExperimentSpanMeta>,
    /// Name of the span.
    #[serde(rename = "name")]
    pub name: String,
    /// Project ID associated with this span.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// Unique identifier of the span.
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// Start time of the span in nanoseconds since Unix epoch.
    #[serde(rename = "start_ns")]
    pub start_ns: i64,
    /// Status of the span.
    #[serde(rename = "status")]
    pub status: String,
    /// List of tags associated with the span.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Trace ID for the span.
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentSpan {
    pub fn new(
        dataset_id: String,
        duration: i64,
        name: String,
        project_id: String,
        span_id: String,
        start_ns: i64,
        status: String,
        trace_id: String,
    ) -> LLMObsExperimentSpan {
        LLMObsExperimentSpan {
            dataset_id,
            duration,
            meta: None,
            name,
            project_id,
            span_id,
            start_ns,
            status,
            tags: None,
            trace_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::LLMObsExperimentSpanMeta) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentSpanVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentSpanVisitor {
            type Value = LLMObsExperimentSpan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_id: Option<String> = None;
                let mut duration: Option<i64> = None;
                let mut meta: Option<crate::datadogV2::model::LLMObsExperimentSpanMeta> = None;
                let mut name: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut start_ns: Option<i64> = None;
                let mut status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut trace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let start_ns = start_ns.ok_or_else(|| M::Error::missing_field("start_ns"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let trace_id = trace_id.ok_or_else(|| M::Error::missing_field("trace_id"))?;

                let content = LLMObsExperimentSpan {
                    dataset_id,
                    duration,
                    meta,
                    name,
                    project_id,
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

        deserializer.deserialize_any(LLMObsExperimentSpanVisitor)
    }
}
