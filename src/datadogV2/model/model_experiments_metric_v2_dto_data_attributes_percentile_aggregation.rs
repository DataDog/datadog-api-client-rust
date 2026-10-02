// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Source measure and settings for a percentile metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricV2DTODataAttributesPercentileAggregation {
    /// Datadog source and query that supply values for the metric.
    #[serde(rename = "datadog_metric_measure")]
    pub datadog_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure>,
    /// Percentile calculated from the selected measure.
    #[serde(rename = "percentile")]
    pub percentile: Option<f64>,
    /// Suffix used to identify this value in pipeline output columns.
    #[serde(rename = "pipeline_column_suffix")]
    pub pipeline_column_suffix: Option<String>,
    /// Filters applied to the metric aggregation.
    #[serde(rename = "property_filters")]
    pub property_filters: Option<Vec<Vec<crate::datadogV2::model::ExperimentsMetricPropertyFilter>>>,
    /// Warehouse measure that supplies values for the metric.
    #[serde(rename = "warehouse_metric_measure")]
    pub warehouse_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsMetricV2DTODataAttributesPercentileAggregation {
    pub fn new() -> ExperimentsMetricV2DTODataAttributesPercentileAggregation {
        ExperimentsMetricV2DTODataAttributesPercentileAggregation {
            datadog_metric_measure: None,
            percentile: None,
            pipeline_column_suffix: None,
            property_filters: None,
            warehouse_metric_measure: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn datadog_metric_measure(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure,
    ) -> Self {
        self.datadog_metric_measure = Some(value);
        self
    }

    pub fn percentile(mut self, value: f64) -> Self {
        self.percentile = Some(value);
        self
    }

    pub fn pipeline_column_suffix(mut self, value: String) -> Self {
        self.pipeline_column_suffix = Some(value);
        self
    }

    pub fn property_filters(
        mut self,
        value: Vec<Vec<crate::datadogV2::model::ExperimentsMetricPropertyFilter>>,
    ) -> Self {
        self.property_filters = Some(value);
        self
    }

    pub fn warehouse_metric_measure(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure,
    ) -> Self {
        self.warehouse_metric_measure = Some(value);
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

impl Default for ExperimentsMetricV2DTODataAttributesPercentileAggregation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricV2DTODataAttributesPercentileAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricV2DTODataAttributesPercentileAggregationVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricV2DTODataAttributesPercentileAggregationVisitor {
            type Value = ExperimentsMetricV2DTODataAttributesPercentileAggregation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut datadog_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure> = None;
                let mut percentile: Option<f64> = None;
                let mut pipeline_column_suffix: Option<String> = None;
                let mut property_filters: Option<
                    Vec<Vec<crate::datadogV2::model::ExperimentsMetricPropertyFilter>>,
                > = None;
                let mut warehouse_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "datadog_metric_measure" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_metric_measure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentile" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            percentile = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_column_suffix" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_column_suffix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            property_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_metric_measure" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse_metric_measure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricV2DTODataAttributesPercentileAggregation {
                    datadog_metric_measure,
                    percentile,
                    pipeline_column_suffix,
                    property_filters,
                    warehouse_metric_measure,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsMetricV2DTODataAttributesPercentileAggregationVisitor)
    }
}
