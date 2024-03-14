// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the definition of a metric tag configuration to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricTagConfigurationUpdateAttributes {
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
    /// When set to true, the configuration will exclude the configured tags and include any other submitted tags.
    /// When set to false, the configuration will include the configured tags and exclude any other submitted tags.
    /// Defaults to false. Requires `tags` property.
    #[serde(rename = "exclude_tags_mode")]
    pub exclude_tags_mode: Option<bool>,
    /// Toggle to include/exclude percentiles for a distribution metric.
    /// Defaults to false. Can only be applied to metrics that have a `metric_type` of `distribution`.
    #[serde(rename = "include_percentiles")]
    pub include_percentiles: Option<bool>,
    /// A list of tag keys that will be queryable for your metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricTagConfigurationUpdateAttributes {
    pub fn new() -> MetricTagConfigurationUpdateAttributes {
        MetricTagConfigurationUpdateAttributes {
            aggregations: None,
            exclude_tags_mode: None,
            include_percentiles: None,
            tags: None,
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

    pub fn exclude_tags_mode(mut self, value: bool) -> Self {
        self.exclude_tags_mode = Some(value);
        self
    }

    pub fn include_percentiles(mut self, value: bool) -> Self {
        self.include_percentiles = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for MetricTagConfigurationUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricTagConfigurationUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricTagConfigurationUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for MetricTagConfigurationUpdateAttributesVisitor {
            type Value = MetricTagConfigurationUpdateAttributes;

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
                let mut exclude_tags_mode: Option<bool> = None;
                let mut include_percentiles: Option<bool> = None;
                let mut tags: Option<Vec<String>> = None;
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
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricTagConfigurationUpdateAttributes {
                    aggregations,
                    exclude_tags_mode,
                    include_percentiles,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricTagConfigurationUpdateAttributesVisitor)
    }
}
