// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A metric to submit to Datadog.
/// See [Datadog metrics](<https://docs.datadoghq.com/developers/metrics/#custom-metrics-properties>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSeries {
    /// If the type of the metric is rate or count, define the corresponding interval.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Metadata for the metric.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::MetricMetadata>,
    /// The name of the timeseries.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Points relating to a metric. All points must be objects with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
    #[serde(rename = "points")]
    pub points: Vec<crate::datadogV2::model::MetricPoint>,
    /// A list of resources to associate with this metric.
    #[serde(rename = "resources")]
    pub resources: Option<Vec<crate::datadogV2::model::MetricResource>>,
    /// The source type name.
    #[serde(rename = "source_type_name")]
    pub source_type_name: Option<String>,
    /// A list of tags associated with the metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of metric. The available types are `0` (unspecified), `1` (count), `2` (rate), and `3` (gauge).
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricIntakeType>,
    /// The unit of point value.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
}

impl MetricSeries {
    pub fn new(metric: String, points: Vec<crate::datadogV2::model::MetricPoint>) -> MetricSeries {
        MetricSeries {
            interval: None,
            metadata: None,
            metric,
            points,
            resources: None,
            source_type_name: None,
            tags: None,
            type_: None,
            unit: None,
        }
    }

    pub fn interval(&mut self, value: i64) -> &mut Self {
        self.interval = Some(value);
        self
    }

    pub fn metadata(&mut self, value: crate::datadogV2::model::MetricMetadata) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn resources(&mut self, value: Vec<crate::datadogV2::model::MetricResource>) -> &mut Self {
        self.resources = Some(value);
        self
    }

    pub fn source_type_name(&mut self, value: String) -> &mut Self {
        self.source_type_name = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::MetricIntakeType) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit(&mut self, value: String) -> &mut Self {
        self.unit = Some(value);
        self
    }
}
