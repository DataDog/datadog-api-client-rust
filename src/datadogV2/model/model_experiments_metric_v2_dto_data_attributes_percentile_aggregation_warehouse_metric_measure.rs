// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Warehouse measure that supplies values for the metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure {
    /// ID of the measure.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Suffix used to identify this value in pipeline output columns.
    #[serde(rename = "pipeline_column_suffix")]
    pub pipeline_column_suffix: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure {
    pub fn new() -> ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure
    {
        ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure {
            id: None,
            pipeline_column_suffix: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn pipeline_column_suffix(mut self, value: String) -> Self {
        self.pipeline_column_suffix = Some(value);
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

impl Default for ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasureVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasureVisitor {
            type Value = ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut pipeline_column_suffix: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "pipeline_column_suffix" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_column_suffix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure {
                    id,
                    pipeline_column_suffix,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasureVisitor,
        )
    }
}
