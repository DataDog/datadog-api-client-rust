// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Object with all metric related metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricMetadata {
    /// Metric description.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Name of the integration that sent the metric if applicable.
    #[serde(rename = "integration", default, with = "::serde_with::rust::double_option")]
    pub integration: Option<Option<String>>,
    /// Per unit of the metric such as `second` in `bytes per second`.
    #[serde(rename = "per_unit", default, with = "::serde_with::rust::double_option")]
    pub per_unit: Option<Option<String>>,
    /// A more human-readable and abbreviated version of the metric name.
    #[serde(rename = "short_name", default, with = "::serde_with::rust::double_option")]
    pub short_name: Option<Option<String>>,
    /// StatsD flush interval of the metric in seconds if applicable.
    #[serde(rename = "statsd_interval", default, with = "::serde_with::rust::double_option")]
    pub statsd_interval: Option<Option<i64>>,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option")]
    pub type_: Option<Option<String>>,
    /// Primary unit of the metric such as `byte` or `operation`.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn integration(mut self, value: Option<String>) -> Self {
        self.integration = Some(value);
        self
    }
    
    pub fn per_unit(mut self, value: Option<String>) -> Self {
        self.per_unit = Some(value);
        self
    }
    
    pub fn short_name(mut self, value: Option<String>) -> Self {
        self.short_name = Some(value);
        self
    }
    
    pub fn statsd_interval(mut self, value: Option<i64>) -> Self {
        self.statsd_interval = Some(value);
        self
    }
    
    pub fn type_(mut self, value: Option<String>) -> Self {
        self.type_ = Some(value);
        self
    }
    
    pub fn unit(mut self, value: Option<String>) -> Self {
        self.unit = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
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
                let mut description: Option<Option<String>> = None;
                let mut integration: Option<Option<String>> = None;
                let mut per_unit: Option<Option<String>> = None;
                let mut short_name: Option<Option<String>> = None;
                let mut statsd_interval: Option<Option<i64>> = None;
                let mut type_: Option<Option<String>> = None;
                let mut unit: Option<Option<String>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "integration" => {
                            integration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "per_unit" => {
                            per_unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "short_name" => {
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "statsd_interval" => {
                            statsd_interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
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
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricMetadataVisitor)
    }
}
