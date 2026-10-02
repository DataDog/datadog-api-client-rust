// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Source measure and aggregation settings for a metric value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
    /// Stored aging threshold in days. The subject aging filter uses the aggregation window end and unit.
    #[serde(rename = "aging_threshold_days")]
    pub aging_threshold_days: Option<i64>,
    /// Datadog source and query that supply values for the metric.
    #[serde(rename = "datadog_metric_measure")]
    pub datadog_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure>,
    /// Whether to exclude subjects whose observation time is shorter than the aggregation window.
    #[serde(rename = "enable_aging_subject_filter")]
    pub enable_aging_subject_filter: Option<bool>,
    /// Aggregation applied to the selected measure.
    #[serde(rename = "operation")]
    pub operation: Option<String>,
    /// Suffix used to identify this value in pipeline output columns.
    #[serde(rename = "pipeline_column_suffix")]
    pub pipeline_column_suffix: Option<String>,
    /// Filters applied to the metric aggregation.
    #[serde(rename = "property_filters")]
    pub property_filters: Option<Vec<Vec<crate::datadogV2::model::ExperimentsMetricPropertyFilter>>>,
    /// Aggregation used to evaluate the threshold.
    #[serde(rename = "threshold_aggregation_type")]
    pub threshold_aggregation_type: Option<String>,
    /// Value used to determine whether the threshold is breached.
    #[serde(rename = "threshold_breach_value")]
    pub threshold_breach_value: Option<f64>,
    /// Comparison applied between the aggregated value and the threshold.
    #[serde(rename = "threshold_comparison_operator")]
    pub threshold_comparison_operator: Option<String>,
    /// Time unit used for the threshold evaluation window.
    #[serde(rename = "threshold_timeframe_dimension")]
    pub threshold_timeframe_dimension: Option<String>,
    /// Size of the threshold evaluation window.
    #[serde(rename = "threshold_timeframe_value")]
    pub threshold_timeframe_value: Option<f64>,
    /// End of the aggregation window in the specified time unit.
    #[serde(rename = "timeframe_end_value")]
    pub timeframe_end_value: Option<f64>,
    /// Start of the aggregation window in the specified time unit.
    #[serde(rename = "timeframe_start_value")]
    pub timeframe_start_value: Option<f64>,
    /// Time unit used for the aggregation window.
    #[serde(rename = "timeframe_unit")]
    pub timeframe_unit: Option<String>,
    /// Warehouse measure that supplies values for the metric.
    #[serde(rename = "warehouse_metric_measure")]
    pub warehouse_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure>,
    /// Fixed lower bound used to cap metric values.
    #[serde(rename = "winsor_lower_fixed_value")]
    pub winsor_lower_fixed_value: Option<f64>,
    /// Percentile used to determine the lower bound for capped metric values.
    #[serde(rename = "winsor_lower_percentile")]
    pub winsor_lower_percentile: Option<f64>,
    /// Fixed upper bound used to cap metric values.
    #[serde(rename = "winsor_upper_fixed_value")]
    pub winsor_upper_fixed_value: Option<f64>,
    /// Percentile used to determine the upper bound for capped metric values.
    #[serde(rename = "winsor_upper_percentile")]
    pub winsor_upper_percentile: Option<f64>,
    /// Method used to cap extreme metric values.
    #[serde(rename = "winsorization_strategy")]
    pub winsorization_strategy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
    pub fn new() -> ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
        ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
            aging_threshold_days: None,
            datadog_metric_measure: None,
            enable_aging_subject_filter: None,
            operation: None,
            pipeline_column_suffix: None,
            property_filters: None,
            threshold_aggregation_type: None,
            threshold_breach_value: None,
            threshold_comparison_operator: None,
            threshold_timeframe_dimension: None,
            threshold_timeframe_value: None,
            timeframe_end_value: None,
            timeframe_start_value: None,
            timeframe_unit: None,
            warehouse_metric_measure: None,
            winsor_lower_fixed_value: None,
            winsor_lower_percentile: None,
            winsor_upper_fixed_value: None,
            winsor_upper_percentile: None,
            winsorization_strategy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aging_threshold_days(mut self, value: i64) -> Self {
        self.aging_threshold_days = Some(value);
        self
    }

    pub fn datadog_metric_measure(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure,
    ) -> Self {
        self.datadog_metric_measure = Some(value);
        self
    }

    pub fn enable_aging_subject_filter(mut self, value: bool) -> Self {
        self.enable_aging_subject_filter = Some(value);
        self
    }

    pub fn operation(mut self, value: String) -> Self {
        self.operation = Some(value);
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

    pub fn threshold_aggregation_type(mut self, value: String) -> Self {
        self.threshold_aggregation_type = Some(value);
        self
    }

    pub fn threshold_breach_value(mut self, value: f64) -> Self {
        self.threshold_breach_value = Some(value);
        self
    }

    pub fn threshold_comparison_operator(mut self, value: String) -> Self {
        self.threshold_comparison_operator = Some(value);
        self
    }

    pub fn threshold_timeframe_dimension(mut self, value: String) -> Self {
        self.threshold_timeframe_dimension = Some(value);
        self
    }

    pub fn threshold_timeframe_value(mut self, value: f64) -> Self {
        self.threshold_timeframe_value = Some(value);
        self
    }

    pub fn timeframe_end_value(mut self, value: f64) -> Self {
        self.timeframe_end_value = Some(value);
        self
    }

    pub fn timeframe_start_value(mut self, value: f64) -> Self {
        self.timeframe_start_value = Some(value);
        self
    }

    pub fn timeframe_unit(mut self, value: String) -> Self {
        self.timeframe_unit = Some(value);
        self
    }

    pub fn warehouse_metric_measure(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure,
    ) -> Self {
        self.warehouse_metric_measure = Some(value);
        self
    }

    pub fn winsor_lower_fixed_value(mut self, value: f64) -> Self {
        self.winsor_lower_fixed_value = Some(value);
        self
    }

    pub fn winsor_lower_percentile(mut self, value: f64) -> Self {
        self.winsor_lower_percentile = Some(value);
        self
    }

    pub fn winsor_upper_fixed_value(mut self, value: f64) -> Self {
        self.winsor_upper_fixed_value = Some(value);
        self
    }

    pub fn winsor_upper_percentile(mut self, value: f64) -> Self {
        self.winsor_upper_percentile = Some(value);
        self
    }

    pub fn winsorization_strategy(mut self, value: String) -> Self {
        self.winsorization_strategy = Some(value);
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

impl Default for ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricV2DTODataAttributesNumeratorAggregationVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricV2DTODataAttributesNumeratorAggregationVisitor {
            type Value = ExperimentsMetricV2DTODataAttributesNumeratorAggregation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aging_threshold_days: Option<i64> = None;
                let mut datadog_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationDatadogMetricMeasure> = None;
                let mut enable_aging_subject_filter: Option<bool> = None;
                let mut operation: Option<String> = None;
                let mut pipeline_column_suffix: Option<String> = None;
                let mut property_filters: Option<
                    Vec<Vec<crate::datadogV2::model::ExperimentsMetricPropertyFilter>>,
                > = None;
                let mut threshold_aggregation_type: Option<String> = None;
                let mut threshold_breach_value: Option<f64> = None;
                let mut threshold_comparison_operator: Option<String> = None;
                let mut threshold_timeframe_dimension: Option<String> = None;
                let mut threshold_timeframe_value: Option<f64> = None;
                let mut timeframe_end_value: Option<f64> = None;
                let mut timeframe_start_value: Option<f64> = None;
                let mut timeframe_unit: Option<String> = None;
                let mut warehouse_metric_measure: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregationWarehouseMetricMeasure> = None;
                let mut winsor_lower_fixed_value: Option<f64> = None;
                let mut winsor_lower_percentile: Option<f64> = None;
                let mut winsor_upper_fixed_value: Option<f64> = None;
                let mut winsor_upper_percentile: Option<f64> = None;
                let mut winsorization_strategy: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aging_threshold_days" => {
                            if v.is_null() {
                                continue;
                            }
                            aging_threshold_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_metric_measure" => {
                            if v.is_null() {
                                continue;
                            }
                            datadog_metric_measure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enable_aging_subject_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            enable_aging_subject_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "threshold_aggregation_type" => {
                            if v.is_null() {
                                continue;
                            }
                            threshold_aggregation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold_breach_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            threshold_breach_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold_comparison_operator" => {
                            if v.is_null() {
                                continue;
                            }
                            threshold_comparison_operator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold_timeframe_dimension" => {
                            if v.is_null() {
                                continue;
                            }
                            threshold_timeframe_dimension =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold_timeframe_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            threshold_timeframe_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe_end_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            timeframe_end_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe_start_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            timeframe_start_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe_unit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_metric_measure" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse_metric_measure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winsor_lower_fixed_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            winsor_lower_fixed_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winsor_lower_percentile" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            winsor_lower_percentile =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winsor_upper_fixed_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            winsor_upper_fixed_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winsor_upper_percentile" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            winsor_upper_percentile =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "winsorization_strategy" => {
                            if v.is_null() {
                                continue;
                            }
                            winsorization_strategy =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricV2DTODataAttributesNumeratorAggregation {
                    aging_threshold_days,
                    datadog_metric_measure,
                    enable_aging_subject_filter,
                    operation,
                    pipeline_column_suffix,
                    property_filters,
                    threshold_aggregation_type,
                    threshold_breach_value,
                    threshold_comparison_operator,
                    threshold_timeframe_dimension,
                    threshold_timeframe_value,
                    timeframe_end_value,
                    timeframe_start_value,
                    timeframe_unit,
                    warehouse_metric_measure,
                    winsor_lower_fixed_value,
                    winsor_lower_percentile,
                    winsor_upper_fixed_value,
                    winsor_upper_percentile,
                    winsorization_strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsMetricV2DTODataAttributesNumeratorAggregationVisitor)
    }
}
