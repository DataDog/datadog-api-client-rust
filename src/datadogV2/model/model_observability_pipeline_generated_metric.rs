// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a log-based custom metric, including its name, type, filter, value computation strategy,
/// and optional grouping fields.
///
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGeneratedMetric {
    /// Optional fields used to group the metric series.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Datadog filter query to match logs for metric generation.
    #[serde(rename = "include")]
    pub include: String,
    /// Type of metric to create.
    #[serde(rename = "metric_type")]
    pub metric_type: crate::datadogV2::model::ObservabilityPipelineGeneratedMetricMetricType,
    /// Name of the custom metric to be created.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies how the value of the generated metric is computed.
    #[serde(rename = "value")]
    pub value: crate::datadogV2::model::ObservabilityPipelineMetricValue,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGeneratedMetric {
    pub fn new(
        include: String,
        metric_type: crate::datadogV2::model::ObservabilityPipelineGeneratedMetricMetricType,
        name: String,
        value: crate::datadogV2::model::ObservabilityPipelineMetricValue,
    ) -> ObservabilityPipelineGeneratedMetric {
        ObservabilityPipelineGeneratedMetric {
            group_by: None,
            include,
            metric_type,
            name,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(mut self, value: Vec<String>) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGeneratedMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGeneratedMetricVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGeneratedMetricVisitor {
            type Value = ObservabilityPipelineGeneratedMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_by: Option<Vec<String>> = None;
                let mut include: Option<String> = None;
                let mut metric_type: Option<
                    crate::datadogV2::model::ObservabilityPipelineGeneratedMetricMetricType,
                > = None;
                let mut name: Option<String> = None;
                let mut value: Option<crate::datadogV2::model::ObservabilityPipelineMetricValue> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_type" => {
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric_type) = metric_type {
                                match _metric_type {
                                    crate::datadogV2::model::ObservabilityPipelineGeneratedMetricMetricType::UnparsedObject(_metric_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::ObservabilityPipelineMetricValue::UnparsedObject(_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let metric_type =
                    metric_type.ok_or_else(|| M::Error::missing_field("metric_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ObservabilityPipelineGeneratedMetric {
                    group_by,
                    include,
                    metric_type,
                    name,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGeneratedMetricVisitor)
    }
}
