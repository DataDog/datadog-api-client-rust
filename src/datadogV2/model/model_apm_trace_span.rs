// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single APM span returned as part of a trace.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct APMTraceSpan {
    /// The duration of the span, in nanoseconds.
    #[serde(rename = "duration")]
    pub duration: i64,
    /// The end time of the span, in Unix nanoseconds.
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// Error flag for a span. `1` when the span is in error, `0` otherwise.
    #[serde(rename = "error")]
    pub error: crate::datadogV2::model::APMSpanErrorFlag,
    /// String-valued tags attached to the span. Tag keys starting with `_` are
    /// filtered out of the response.
    #[serde(rename = "meta")]
    pub meta: std::collections::BTreeMap<String, String>,
    /// Numeric metrics attached to the span. Metric keys starting with `_` are
    /// filtered out of the response.
    #[serde(rename = "metrics")]
    pub metrics: std::collections::BTreeMap<String, f64>,
    /// The operation name of the span.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the parent span, or `0` when the span is a trace root.
    #[serde(rename = "parentID")]
    pub parent_id: i64,
    /// The resource that the span describes.
    #[serde(rename = "resource")]
    pub resource: String,
    /// A hash of the resource field.
    #[serde(rename = "resourceHash")]
    pub resource_hash: Option<String>,
    /// Whether access to the span is restricted by the organization's data access policies.
    #[serde(rename = "restricted")]
    pub restricted: Option<bool>,
    /// The time spent in the span itself, excluding time spent in child spans, in nanoseconds.
    #[serde(rename = "self_time")]
    pub self_time: Option<f64>,
    /// The name of the service that emitted the span.
    #[serde(rename = "service")]
    pub service: String,
    /// The span ID, as an unsigned 64-bit integer.
    #[serde(rename = "spanID")]
    pub span_id: i64,
    /// The start time of the span, in Unix nanoseconds.
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// The lower 64 bits of the trace ID, as an unsigned 64-bit integer.
    #[serde(rename = "traceID")]
    pub trace_id: i64,
    /// The full 128-bit trace ID, encoded as a 32-character hexadecimal string.
    #[serde(rename = "traceIDFull")]
    pub trace_id_full: String,
    /// The type of the span (for example, `web`, `db`, or `rpc`).
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl APMTraceSpan {
    pub fn new(
        duration: i64,
        end_time: i64,
        error: crate::datadogV2::model::APMSpanErrorFlag,
        meta: std::collections::BTreeMap<String, String>,
        metrics: std::collections::BTreeMap<String, f64>,
        name: String,
        parent_id: i64,
        resource: String,
        service: String,
        span_id: i64,
        start_time: i64,
        trace_id: i64,
        trace_id_full: String,
        type_: String,
    ) -> APMTraceSpan {
        APMTraceSpan {
            duration,
            end_time,
            error,
            meta,
            metrics,
            name,
            parent_id,
            resource,
            resource_hash: None,
            restricted: None,
            self_time: None,
            service,
            span_id,
            start_time,
            trace_id,
            trace_id_full,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn resource_hash(mut self, value: String) -> Self {
        self.resource_hash = Some(value);
        self
    }

    pub fn restricted(mut self, value: bool) -> Self {
        self.restricted = Some(value);
        self
    }

    pub fn self_time(mut self, value: f64) -> Self {
        self.self_time = Some(value);
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

impl<'de> Deserialize<'de> for APMTraceSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APMTraceSpanVisitor;
        impl<'a> Visitor<'a> for APMTraceSpanVisitor {
            type Value = APMTraceSpan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<i64> = None;
                let mut end_time: Option<i64> = None;
                let mut error: Option<crate::datadogV2::model::APMSpanErrorFlag> = None;
                let mut meta: Option<std::collections::BTreeMap<String, String>> = None;
                let mut metrics: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut name: Option<String> = None;
                let mut parent_id: Option<i64> = None;
                let mut resource: Option<String> = None;
                let mut resource_hash: Option<String> = None;
                let mut restricted: Option<bool> = None;
                let mut self_time: Option<f64> = None;
                let mut service: Option<String> = None;
                let mut span_id: Option<i64> = None;
                let mut start_time: Option<i64> = None;
                let mut trace_id: Option<i64> = None;
                let mut trace_id_full: Option<String> = None;
                let mut type_: Option<String> = None;
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
                        "resourceHash" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_hash =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restricted" => {
                            if v.is_null() {
                                continue;
                            }
                            restricted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "self_time" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            self_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spanID" => {
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startTime" => {
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceID" => {
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceIDFull" => {
                            trace_id_full =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let duration = duration.ok_or_else(|| M::Error::missing_field("duration"))?;
                let end_time = end_time.ok_or_else(|| M::Error::missing_field("end_time"))?;
                let error = error.ok_or_else(|| M::Error::missing_field("error"))?;
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;
                let metrics = metrics.ok_or_else(|| M::Error::missing_field("metrics"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let parent_id = parent_id.ok_or_else(|| M::Error::missing_field("parent_id"))?;
                let resource = resource.ok_or_else(|| M::Error::missing_field("resource"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let span_id = span_id.ok_or_else(|| M::Error::missing_field("span_id"))?;
                let start_time = start_time.ok_or_else(|| M::Error::missing_field("start_time"))?;
                let trace_id = trace_id.ok_or_else(|| M::Error::missing_field("trace_id"))?;
                let trace_id_full =
                    trace_id_full.ok_or_else(|| M::Error::missing_field("trace_id_full"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = APMTraceSpan {
                    duration,
                    end_time,
                    error,
                    meta,
                    metrics,
                    name,
                    parent_id,
                    resource,
                    resource_hash,
                    restricted,
                    self_time,
                    service,
                    span_id,
                    start_time,
                    trace_id,
                    trace_id_full,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(APMTraceSpanVisitor)
    }
}
