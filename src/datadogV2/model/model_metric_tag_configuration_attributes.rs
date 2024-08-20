// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric tag configuration attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricTagConfigurationAttributes {
    /// A list of queryable aggregation combinations for a count, rate, or gauge metric.
    /// By default, count and rate metrics require the (time: sum, space: sum) aggregation and
    /// Gauge metrics require the (time: avg, space: avg) aggregation.
    /// Additional time & space combinations are also available:
    ///
    /// - time: avg, space: avg
    /// - time: avg, space: max
    /// - time: avg, space: min
    /// - time: avg, space: sum
    /// - time: count, space: sum
    /// - time: max, space: max
    /// - time: min, space: min
    /// - time: sum, space: avg
    /// - time: sum, space: sum
    ///
    /// Can only be applied to metrics that have a `metric_type` of `count`, `rate`, or `gauge`.
    #[serde(rename = "aggregations")]
    pub aggregations: Option<Vec<crate::datadogV2::model::MetricCustomAggregation>>,
    /// Timestamp when the tag configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    /// Defaults to false. Requires `tags` property.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
    /// Only present when the `metric_type` is `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// The metric's type.
    #[serde(rename = "metric_type")]
    pub metric_type: Option<crate::datadogV2::model::MetricTagConfigurationMetricTypes>,
    /// Timestamp when the tag configuration was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// List of tag keys on which to group.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricTagConfigurationAttributes {
    pub fn new() -> MetricTagConfigurationAttributes {
        MetricTagConfigurationAttributes {
            aggregations: None,
            created_at: None,
            exclude_tags_mode: None,
            include_percentiles: None,
            metric_type: None,
            modified_at: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregations(
        mut self,
        value: Vec<crate::datadogV2::model::MetricCustomAggregation>,
    ) -> Self {
        self.aggregations = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn exclude_tags_mode(mut self, value: bool) -> Self {
        self.exclude_tags_mode = Some(value);
        self
    }

    pub fn include_percentiles(mut self, value: bool) -> Self {
        self.include_percentiles = Some(value);
        self
    }

    pub fn metric_type(
        mut self,
        value: crate::datadogV2::model::MetricTagConfigurationMetricTypes,
    ) -> Self {
        self.metric_type = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
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

impl Default for MetricTagConfigurationAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricTagConfigurationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricTagConfigurationAttributesVisitor;
        impl<'a> Visitor<'a> for MetricTagConfigurationAttributesVisitor {
            type Value = MetricTagConfigurationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregations: Option<
                    Vec<crate::datadogV2::model::MetricCustomAggregation>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut exclude_tags_mode: Option<bool> = None;
                let mut include_percentiles: Option<bool> = None;
                let mut metric_type: Option<
                    crate::datadogV2::model::MetricTagConfigurationMetricTypes,
                > = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregations" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclude_tags_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_tags_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_percentiles" => {
                            if v.is_null() {
                                continue;
                            }
                            include_percentiles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_type" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric_type) = metric_type {
                                match _metric_type {
                                    crate::datadogV2::model::MetricTagConfigurationMetricTypes::UnparsedObject(_metric_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MetricTagConfigurationAttributes {
                    aggregations,
                    created_at,
                    exclude_tags_mode,
                    include_percentiles,
                    metric_type,
                    modified_at,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricTagConfigurationAttributesVisitor)
    }
}
