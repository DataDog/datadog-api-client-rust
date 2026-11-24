// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for one metric. Every attribute is always present. A value that is unknown or not set is `null`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricMetadataAttributes {
    /// Description of the metric, or `null` if none is set.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether historical metrics ingestion is enabled for the metric.
    #[serde(rename = "historical_metrics_enabled")]
    pub historical_metrics_enabled: bool,
    /// Whether percentile aggregations are enabled for the metric. Percentile aggregations are available only for distribution metrics.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: bool,
    /// Name of the integration that sent the metric, or `null` if none is set. This is always `null` for
    /// out-of-the-box RUM metrics.
    #[serialize_always]
    #[serde(rename = "integration")]
    pub integration: Option<String>,
    /// When the metric's tag or percentile configuration was last changed, or `null` if it has never been configured.
    #[serialize_always]
    #[serde(rename = "last_configured_at")]
    pub last_configured_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Metric type, or `null` if it isn't known. Known values are `gauge`, `count`, `rate`, `distribution`,
    /// `explicit_histogram`, and `exponential_histogram`, plus the legacy values `counter` and `hll`. New values may
    /// be added. For histograms and sketches, this can differ from the type returned by
    /// `GET /api/v1/metrics/{metric_name}`.
    #[serialize_always]
    #[serde(rename = "metric_type")]
    pub metric_type: Option<String>,
    /// Where the metric comes from. Each value is the display label shown on the Metrics Summary page. The set of labels can grow as Datadog adds products. Each field is `null` when it isn't known.
    #[serde(rename = "origin")]
    pub origin: crate::datadogV2::model::MetricMetadataOrigin,
    /// A unit of measure from the Datadog unit catalog, or `null` when no unit is set.
    #[serialize_always]
    #[serde(rename = "per_unit")]
    pub per_unit: Option<crate::datadogV2::model::MetricMetadataUnit>,
    /// Short display name of the metric, or `null` if none is set.
    #[serialize_always]
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    /// How the metric reaches Datadog, or `null` if it isn't known. Known values are `intake`, `api_default`, and
    /// `intake_serverless` for submitted metrics, and `api_logs`, `api_processes`, `api_apm_spans`, and `api_rum`
    /// for metrics generated from logs, processes, APM spans, and RUM events. New values may be added.
    #[serialize_always]
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// StatsD flush interval of the metric in seconds, or `null` if none is set.
    #[serialize_always]
    #[serde(rename = "statsd_interval")]
    pub statsd_interval: Option<i64>,
    /// A unit of measure from the Datadog unit catalog, or `null` when no unit is set.
    #[serialize_always]
    #[serde(rename = "unit")]
    pub unit: Option<crate::datadogV2::model::MetricMetadataUnit>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricMetadataAttributes {
    pub fn new(
        description: Option<String>,
        historical_metrics_enabled: bool,
        include_percentiles: bool,
        integration: Option<String>,
        last_configured_at: Option<chrono::DateTime<chrono::Utc>>,
        metric_type: Option<String>,
        origin: crate::datadogV2::model::MetricMetadataOrigin,
        per_unit: Option<crate::datadogV2::model::MetricMetadataUnit>,
        short_name: Option<String>,
        source: Option<String>,
        statsd_interval: Option<i64>,
        unit: Option<crate::datadogV2::model::MetricMetadataUnit>,
    ) -> MetricMetadataAttributes {
        MetricMetadataAttributes {
            description,
            historical_metrics_enabled,
            include_percentiles,
            integration,
            last_configured_at,
            metric_type,
            origin,
            per_unit,
            short_name,
            source,
            statsd_interval,
            unit,
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

impl<'de> Deserialize<'de> for MetricMetadataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricMetadataAttributesVisitor;
        impl<'a> Visitor<'a> for MetricMetadataAttributesVisitor {
            type Value = MetricMetadataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut historical_metrics_enabled: Option<bool> = None;
                let mut include_percentiles: Option<bool> = None;
                let mut integration: Option<Option<String>> = None;
                let mut last_configured_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut metric_type: Option<Option<String>> = None;
                let mut origin: Option<crate::datadogV2::model::MetricMetadataOrigin> = None;
                let mut per_unit: Option<Option<crate::datadogV2::model::MetricMetadataUnit>> =
                    None;
                let mut short_name: Option<Option<String>> = None;
                let mut source: Option<Option<String>> = None;
                let mut statsd_interval: Option<Option<i64>> = None;
                let mut unit: Option<Option<crate::datadogV2::model::MetricMetadataUnit>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "historical_metrics_enabled" => {
                            historical_metrics_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_percentiles" => {
                            include_percentiles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_configured_at" => {
                            last_configured_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_type" => {
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin" => {
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "per_unit" => {
                            per_unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "statsd_interval" => {
                            statsd_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let historical_metrics_enabled = historical_metrics_enabled
                    .ok_or_else(|| M::Error::missing_field("historical_metrics_enabled"))?;
                let include_percentiles = include_percentiles
                    .ok_or_else(|| M::Error::missing_field("include_percentiles"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let last_configured_at = last_configured_at
                    .ok_or_else(|| M::Error::missing_field("last_configured_at"))?;
                let metric_type =
                    metric_type.ok_or_else(|| M::Error::missing_field("metric_type"))?;
                let origin = origin.ok_or_else(|| M::Error::missing_field("origin"))?;
                let per_unit = per_unit.ok_or_else(|| M::Error::missing_field("per_unit"))?;
                let short_name = short_name.ok_or_else(|| M::Error::missing_field("short_name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let statsd_interval =
                    statsd_interval.ok_or_else(|| M::Error::missing_field("statsd_interval"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;

                let content = MetricMetadataAttributes {
                    description,
                    historical_metrics_enabled,
                    include_percentiles,
                    integration,
                    last_configured_at,
                    metric_type,
                    origin,
                    per_unit,
                    short_name,
                    source,
                    statsd_interval,
                    unit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricMetadataAttributesVisitor)
    }
}
