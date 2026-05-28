// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A node in the pruned trace tree.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SummarizedSpan {
    /// The child spans of this node in the pruned tree.
    #[serde(rename = "children")]
    pub children: Vec<crate::datadogV2::model::SummarizedSpan>,
    /// The duration of the span, in seconds.
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: f64,
    /// The end time of the span, in RFC3339 format.
    #[serde(rename = "endTime")]
    pub end_time: chrono::DateTime<chrono::Utc>,
    /// Error flag for a span. `1` when the span is in error, `0` otherwise.
    #[serde(rename = "error")]
    pub error: crate::datadogV2::model::APMSpanErrorFlag,
    /// The number of child spans that were pruned from this node when summarizing the trace.
    #[serde(rename = "hidden_child_spans_count")]
    pub hidden_child_spans_count: i32,
    /// String-valued tags attached to the span.
    #[serde(rename = "meta")]
    pub meta: std::collections::BTreeMap<String, String>,
    /// Numeric metrics attached to the span.
    #[serde(rename = "metrics")]
    pub metrics: std::collections::BTreeMap<String, f64>,
    /// The operation name of the span.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the parent span, or `0` when the span is the trace root.
    #[serde(rename = "parentID")]
    pub parent_id: i64,
    /// The resource that the span describes.
    #[serde(rename = "resource")]
    pub resource: String,
    /// The name of the service that emitted the span.
    #[serde(rename = "service")]
    pub service: String,
    /// The span ID, as an unsigned 64-bit integer.
    #[serde(rename = "spanID")]
    pub span_id: i64,
    /// The OpenTelemetry span kind, for example `INTERNAL`, `SERVER`, `CLIENT`,
    /// `PRODUCER`, or `CONSUMER`.
    #[serde(rename = "span_kind")]
    pub span_kind: String,
    /// The start time of the span, in RFC3339 format.
    #[serde(rename = "startTime")]
    pub start_time: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SummarizedSpan {
    pub fn new(
        children: Vec<crate::datadogV2::model::SummarizedSpan>,
        duration_seconds: f64,
        end_time: chrono::DateTime<chrono::Utc>,
        error: crate::datadogV2::model::APMSpanErrorFlag,
        hidden_child_spans_count: i32,
        meta: std::collections::BTreeMap<String, String>,
        metrics: std::collections::BTreeMap<String, f64>,
        name: String,
        parent_id: i64,
        resource: String,
        service: String,
        span_id: i64,
        span_kind: String,
        start_time: chrono::DateTime<chrono::Utc>,
    ) -> SummarizedSpan {
        SummarizedSpan {
            children,
            duration_seconds,
            end_time,
            error,
            hidden_child_spans_count,
            meta,
            metrics,
            name,
            parent_id,
            resource,
            service,
            span_id,
            span_kind,
            start_time,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SummarizedSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SummarizedSpanVisitor;
        impl<'a> Visitor<'a> for SummarizedSpanVisitor {
            type Value = SummarizedSpan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut children: Option<Vec<crate::datadogV2::model::SummarizedSpan>> = None;
                let mut duration_seconds: Option<f64> = None;
                let mut end_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut error: Option<crate::datadogV2::model::APMSpanErrorFlag> = None;
                let mut hidden_child_spans_count: Option<i32> = None;
                let mut meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut metrics: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut name: Option<String> = None;
                let mut parent_id: Option<i64> = None;
                let mut resource: Option<String> = None;
                let mut service: Option<String> = None;
                let mut span_id: Option<i64> = None;
                let mut span_kind: Option<String> = None;
                let mut start_time: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "children" => {
                            children = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "durationSeconds" => {
                            duration_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "endTime" => {
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _error) = error {
                                match _error {
                                    crate::datadogV2::model::APMSpanErrorFlag::UnparsedObject(
                                        _error,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "hidden_child_spans_count" => {
                            hidden_child_spans_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parentID" => {
                            parent_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spanID" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_kind" => {
                            span_kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startTime" => {
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let children = children.ok_or_else(|| M::Error::missing_field("children"))?;
                let duration_seconds =
                    duration_seconds.ok_or_else(|| M::Error::missing_field("duration_seconds"))?;
                let end_time = end_time.ok_or_else(|| M::Error::missing_field("end_time"))?;
                let error = error.ok_or_else(|| M::Error::missing_field("error"))?;
                let hidden_child_spans_count = hidden_child_spans_count
                    .ok_or_else(|| M::Error::missing_field("hidden_child_spans_count"))?;
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;
                let metrics = metrics.ok_or_else(|| M::Error::missing_field("metrics"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let parent_id = parent_id.ok_or_else(|| M::Error::missing_field("parent_id"))?;
                let resource = resource.ok_or_else(|| M::Error::missing_field("resource"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let span_kind = span_kind.ok_or_else(|| M::Error::missing_field("span_kind"))?;
                let start_time = start_time.ok_or_else(|| M::Error::missing_field("start_time"))?;

                let content = SummarizedSpan {
                    children,
                    duration_seconds,
                    end_time,
                    error,
                    hidden_child_spans_count,
                    meta,
                    metrics,
                    name,
                    parent_id,
                    resource,
                    service,
                    span_id,
                    span_kind,
                    start_time,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SummarizedSpanVisitor)
    }
}
