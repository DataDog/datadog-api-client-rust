// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for a percentile aggregation that uses a Datadog measure. The other measure must be omitted or null.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsDatadogPercentileAggregationInput {
    /// Datadog measure used to calculate a percentile.
    #[serde(rename = "datadog_metric_measure")]
    pub datadog_metric_measure: crate::datadogV2::model::ExperimentsDatadogPercentileMeasureInput,
    /// Percentile to calculate from the measure values.
    #[serde(rename = "percentile")]
    pub percentile: f64,
    /// Property filters that select data for the percentile calculation.
    #[serde(
        rename = "property_filters",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub property_filters:
        Option<Option<Vec<Vec<crate::datadogV2::model::ExperimentsPropertyFilterInput>>>>,
    /// Optional warehouse measure. Use null when the other measure is selected.
    #[serde(
        rename = "warehouse_metric_measure",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub warehouse_metric_measure:
        Option<Option<crate::datadogV2::model::ExperimentsNullableWarehouseMetricMeasureInput>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsDatadogPercentileAggregationInput {
    pub fn new(
        datadog_metric_measure: crate::datadogV2::model::ExperimentsDatadogPercentileMeasureInput,
        percentile: f64,
    ) -> ExperimentsDatadogPercentileAggregationInput {
        ExperimentsDatadogPercentileAggregationInput {
            datadog_metric_measure,
            percentile,
            property_filters: None,
            warehouse_metric_measure: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn property_filters(
        mut self,
        value: Option<Vec<Vec<crate::datadogV2::model::ExperimentsPropertyFilterInput>>>,
    ) -> Self {
        self.property_filters = Some(value);
        self
    }

    pub fn warehouse_metric_measure(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsNullableWarehouseMetricMeasureInput>,
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

impl<'de> Deserialize<'de> for ExperimentsDatadogPercentileAggregationInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsDatadogPercentileAggregationInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsDatadogPercentileAggregationInputVisitor {
            type Value = ExperimentsDatadogPercentileAggregationInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut datadog_metric_measure: Option<
                    crate::datadogV2::model::ExperimentsDatadogPercentileMeasureInput,
                > = None;
                let mut percentile: Option<f64> = None;
                let mut property_filters: Option<
                    Option<Vec<Vec<crate::datadogV2::model::ExperimentsPropertyFilterInput>>>,
                > = None;
                let mut warehouse_metric_measure: Option<
                    Option<crate::datadogV2::model::ExperimentsNullableWarehouseMetricMeasureInput>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "datadog_metric_measure" => {
                            datadog_metric_measure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentile" => {
                            percentile = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property_filters" => {
                            property_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_metric_measure" => {
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
                let datadog_metric_measure = datadog_metric_measure
                    .ok_or_else(|| M::Error::missing_field("datadog_metric_measure"))?;
                let percentile = percentile.ok_or_else(|| M::Error::missing_field("percentile"))?;

                let content = ExperimentsDatadogPercentileAggregationInput {
                    datadog_metric_measure,
                    percentile,
                    property_filters,
                    warehouse_metric_measure,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsDatadogPercentileAggregationInputVisitor)
    }
}
