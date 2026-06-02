// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated performance statistics for a single network resource across sampled view instances.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedResource {
    /// Average total duration in milliseconds.
    #[serde(rename = "avg_duration_ms")]
    pub avg_duration_ms: f64,
    /// Average start time relative to view start in milliseconds.
    #[serde(rename = "avg_start_time_ms")]
    pub avg_start_time_ms: f64,
    /// Cache hit rate as a percentage.
    #[serde(rename = "cache_hit_rate_pct")]
    pub cache_hit_rate_pct: f64,
    /// Number of requests served from cache.
    #[serde(rename = "cached_count")]
    pub cached_count: i32,
    /// Number of requests downloaded from the network.
    #[serde(rename = "downloaded_count")]
    pub downloaded_count: i32,
    /// 75th percentile duration across all view names in the application, present when include_global_appearance is true.
    #[serde(rename = "global_p75_duration_ms")]
    pub global_p75_duration_ms: Option<f64>,
    /// Number of distinct view names in the application that load this resource, present when include_global_appearance is true.
    #[serde(rename = "global_view_name_count")]
    pub global_view_name_count: Option<i32>,
    /// Percentage of distinct view names in the application that load this resource, present when include_global_appearance is true.
    #[serde(rename = "global_view_name_pct")]
    pub global_view_name_pct: Option<f64>,
    /// HTTP method for the resource request.
    #[serialize_always]
    #[serde(rename = "http_method")]
    pub http_method: Option<String>,
    /// Percentage of sampled view instances that loaded this resource.
    #[serde(rename = "load_frequency_pct")]
    pub load_frequency_pct: f64,
    /// Maximum duration in milliseconds.
    #[serde(rename = "max_duration_ms")]
    pub max_duration_ms: f64,
    /// Median duration in milliseconds.
    #[serde(rename = "median_duration_ms")]
    pub median_duration_ms: f64,
    /// Minimum duration in milliseconds.
    #[serde(rename = "min_duration_ms")]
    pub min_duration_ms: f64,
    /// 75th percentile duration in milliseconds.
    #[serde(rename = "p75_duration_ms")]
    pub p75_duration_ms: f64,
    /// 95th percentile duration in milliseconds.
    #[serde(rename = "p95_duration_ms")]
    pub p95_duration_ms: f64,
    /// Resource type (JS, CSS, image, fetch, XHR, document, and so on).
    #[serialize_always]
    #[serde(rename = "resource_type")]
    pub resource_type: Option<String>,
    /// URL path group used to aggregate similar resources.
    #[serde(rename = "resource_url_path_group")]
    pub resource_url_path_group: String,
    /// Average timing breakdown per network phase for a resource.
    #[serde(rename = "timing_breakdown")]
    pub timing_breakdown: crate::datadogV2::model::AggregatedResourceTimingBreakdown,
    /// Total number of requests for this resource across all sampled views.
    #[serde(rename = "total_requests")]
    pub total_requests: i32,
    /// Number of sampled view instances that loaded this resource.
    #[serde(rename = "views_with_resource")]
    pub views_with_resource: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedResource {
    pub fn new(
        avg_duration_ms: f64,
        avg_start_time_ms: f64,
        cache_hit_rate_pct: f64,
        cached_count: i32,
        downloaded_count: i32,
        http_method: Option<String>,
        load_frequency_pct: f64,
        max_duration_ms: f64,
        median_duration_ms: f64,
        min_duration_ms: f64,
        p75_duration_ms: f64,
        p95_duration_ms: f64,
        resource_type: Option<String>,
        resource_url_path_group: String,
        timing_breakdown: crate::datadogV2::model::AggregatedResourceTimingBreakdown,
        total_requests: i32,
        views_with_resource: i32,
    ) -> AggregatedResource {
        AggregatedResource {
            avg_duration_ms,
            avg_start_time_ms,
            cache_hit_rate_pct,
            cached_count,
            downloaded_count,
            global_p75_duration_ms: None,
            global_view_name_count: None,
            global_view_name_pct: None,
            http_method,
            load_frequency_pct,
            max_duration_ms,
            median_duration_ms,
            min_duration_ms,
            p75_duration_ms,
            p95_duration_ms,
            resource_type,
            resource_url_path_group,
            timing_breakdown,
            total_requests,
            views_with_resource,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn global_p75_duration_ms(mut self, value: f64) -> Self {
        self.global_p75_duration_ms = Some(value);
        self
    }

    pub fn global_view_name_count(mut self, value: i32) -> Self {
        self.global_view_name_count = Some(value);
        self
    }

    pub fn global_view_name_pct(mut self, value: f64) -> Self {
        self.global_view_name_pct = Some(value);
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

impl<'de> Deserialize<'de> for AggregatedResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedResourceVisitor;
        impl<'a> Visitor<'a> for AggregatedResourceVisitor {
            type Value = AggregatedResource;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_duration_ms: Option<f64> = None;
                let mut avg_start_time_ms: Option<f64> = None;
                let mut cache_hit_rate_pct: Option<f64> = None;
                let mut cached_count: Option<i32> = None;
                let mut downloaded_count: Option<i32> = None;
                let mut global_p75_duration_ms: Option<f64> = None;
                let mut global_view_name_count: Option<i32> = None;
                let mut global_view_name_pct: Option<f64> = None;
                let mut http_method: Option<Option<String>> = None;
                let mut load_frequency_pct: Option<f64> = None;
                let mut max_duration_ms: Option<f64> = None;
                let mut median_duration_ms: Option<f64> = None;
                let mut min_duration_ms: Option<f64> = None;
                let mut p75_duration_ms: Option<f64> = None;
                let mut p95_duration_ms: Option<f64> = None;
                let mut resource_type: Option<Option<String>> = None;
                let mut resource_url_path_group: Option<String> = None;
                let mut timing_breakdown: Option<
                    crate::datadogV2::model::AggregatedResourceTimingBreakdown,
                > = None;
                let mut total_requests: Option<i32> = None;
                let mut views_with_resource: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_duration_ms" => {
                            avg_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_start_time_ms" => {
                            avg_start_time_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cache_hit_rate_pct" => {
                            cache_hit_rate_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cached_count" => {
                            cached_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "downloaded_count" => {
                            downloaded_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_p75_duration_ms" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_p75_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_view_name_count" => {
                            if v.is_null() {
                                continue;
                            }
                            global_view_name_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_view_name_pct" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_view_name_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "http_method" => {
                            http_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "load_frequency_pct" => {
                            load_frequency_pct =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_duration_ms" => {
                            max_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "median_duration_ms" => {
                            median_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_duration_ms" => {
                            min_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "p75_duration_ms" => {
                            p75_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "p95_duration_ms" => {
                            p95_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_url_path_group" => {
                            resource_url_path_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timing_breakdown" => {
                            timing_breakdown =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_requests" => {
                            total_requests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "views_with_resource" => {
                            views_with_resource =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let avg_duration_ms =
                    avg_duration_ms.ok_or_else(|| M::Error::missing_field("avg_duration_ms"))?;
                let avg_start_time_ms = avg_start_time_ms
                    .ok_or_else(|| M::Error::missing_field("avg_start_time_ms"))?;
                let cache_hit_rate_pct = cache_hit_rate_pct
                    .ok_or_else(|| M::Error::missing_field("cache_hit_rate_pct"))?;
                let cached_count =
                    cached_count.ok_or_else(|| M::Error::missing_field("cached_count"))?;
                let downloaded_count =
                    downloaded_count.ok_or_else(|| M::Error::missing_field("downloaded_count"))?;
                let http_method =
                    http_method.ok_or_else(|| M::Error::missing_field("http_method"))?;
                let load_frequency_pct = load_frequency_pct
                    .ok_or_else(|| M::Error::missing_field("load_frequency_pct"))?;
                let max_duration_ms =
                    max_duration_ms.ok_or_else(|| M::Error::missing_field("max_duration_ms"))?;
                let median_duration_ms = median_duration_ms
                    .ok_or_else(|| M::Error::missing_field("median_duration_ms"))?;
                let min_duration_ms =
                    min_duration_ms.ok_or_else(|| M::Error::missing_field("min_duration_ms"))?;
                let p75_duration_ms =
                    p75_duration_ms.ok_or_else(|| M::Error::missing_field("p75_duration_ms"))?;
                let p95_duration_ms =
                    p95_duration_ms.ok_or_else(|| M::Error::missing_field("p95_duration_ms"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let resource_url_path_group = resource_url_path_group
                    .ok_or_else(|| M::Error::missing_field("resource_url_path_group"))?;
                let timing_breakdown =
                    timing_breakdown.ok_or_else(|| M::Error::missing_field("timing_breakdown"))?;
                let total_requests =
                    total_requests.ok_or_else(|| M::Error::missing_field("total_requests"))?;
                let views_with_resource = views_with_resource
                    .ok_or_else(|| M::Error::missing_field("views_with_resource"))?;

                let content = AggregatedResource {
                    avg_duration_ms,
                    avg_start_time_ms,
                    cache_hit_rate_pct,
                    cached_count,
                    downloaded_count,
                    global_p75_duration_ms,
                    global_view_name_count,
                    global_view_name_pct,
                    http_method,
                    load_frequency_pct,
                    max_duration_ms,
                    median_duration_ms,
                    min_duration_ms,
                    p75_duration_ms,
                    p95_duration_ms,
                    resource_type,
                    resource_url_path_group,
                    timing_breakdown,
                    total_requests,
                    views_with_resource,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedResourceVisitor)
    }
}
