// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with all metric related metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricMetadata {
    /// Metric description.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the integration that sent the metric if applicable.
    #[serde(rename = "integration")]
    pub integration: Option<String>,
    /// Per unit of the metric such as `second` in `bytes per second`.
    #[serde(rename = "per_unit")]
    pub per_unit: Option<String>,
    /// A more human-readable and abbreviated version of the metric name.
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    /// StatsD flush interval of the metric in seconds if applicable.
    #[serde(rename = "statsd_interval")]
    pub statsd_interval: Option<i64>,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Primary unit of the metric such as `byte` or `operation`.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricMetadata {
    pub fn new() -> MetricMetadata {
        MetricMetadata {
            description: None,
            integration: None,
            per_unit: None,
            short_name: None,
            statsd_interval: None,
            type_: None,
            unit: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn integration(mut self, value: String) -> Self {
        self.integration = Some(value);
        self
    }

    pub fn per_unit(mut self, value: String) -> Self {
        self.per_unit = Some(value);
        self
    }

    pub fn short_name(mut self, value: String) -> Self {
        self.short_name = Some(value);
        self
    }

    pub fn statsd_interval(mut self, value: i64) -> Self {
        self.statsd_interval = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }
}

impl Default for MetricMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricMetadataVisitor;
        impl<'a> Visitor<'a> for MetricMetadataVisitor {
            type Value = MetricMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut integration: Option<String> = None;
                let mut per_unit: Option<String> = None;
                let mut short_name: Option<String> = None;
                let mut statsd_interval: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut unit: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration" => {
                            if v.is_null() {
                                continue;
                            }
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "per_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            per_unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            if v.is_null() {
                                continue;
                            }
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "statsd_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            statsd_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = MetricMetadata {
                    description,
                    integration,
                    per_unit,
                    short_name,
                    statsd_interval,
                    type_,
                    unit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricMetadataVisitor)
    }
}
