// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric to submit to Datadog.
/// See [Datadog metrics](<https://docs.datadoghq.com/developers/metrics/#custom-metrics-properties>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metadata(mut self, value: crate::datadogV2::model::MetricMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn resources(mut self, value: Vec<crate::datadogV2::model::MetricResource>) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn source_type_name(mut self, value: String) -> Self {
        self.source_type_name = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::MetricIntakeType) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MetricSeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricSeriesVisitor;
        impl<'a> Visitor<'a> for MetricSeriesVisitor {
            type Value = MetricSeries;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interval: Option<i64> = None;
                let mut metadata: Option<crate::datadogV2::model::MetricMetadata> = None;
                let mut metric: Option<String> = None;
                let mut points: Option<Vec<crate::datadogV2::model::MetricPoint>> = None;
                let mut resources: Option<Vec<crate::datadogV2::model::MetricResource>> = None;
                let mut source_type_name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::MetricIntakeType> = None;
                let mut unit: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "points" => {
                            points = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resources" => {
                            if v.is_null() {
                                continue;
                            }
                            resources = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_type_name" => {
                            if v.is_null() {
                                continue;
                            }
                            source_type_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MetricIntakeType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;
                let points = points.ok_or_else(|| M::Error::missing_field("points"))?;

                let content = MetricSeries {
                    interval,
                    metadata,
                    metric,
                    points,
                    resources,
                    source_type_name,
                    tags,
                    type_,
                    unit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricSeriesVisitor)
    }
}
