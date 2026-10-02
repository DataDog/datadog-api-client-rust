// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings for a metric aggregation that uses a Warehouse measure. The other measure must be omitted or null.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsWarehouseMetricAggregationInput {
    /// Stored aging threshold in days. The subject aging filter uses the aggregation window end and unit.
    #[serde(rename = "aging_threshold_days")]
    pub aging_threshold_days: Option<i64>,
    /// Optional Datadog measure. Use null when the other measure is selected.
    #[serde(rename = "datadog_metric_measure", default, with = "::serde_with::rust::double_option")]
    pub datadog_metric_measure: Option<Option<crate::datadogV2::model::ExperimentsNullableDatadogMetricMeasureInput>>,
    /// Whether to exclude subjects whose observation time is shorter than the aggregation window.
    #[serde(rename = "enable_aging_subject_filter")]
    pub enable_aging_subject_filter: Option<bool>,
    /// Calculation applied to the measure values, such as sum.
    #[serde(rename = "operation")]
    pub operation: String,
    /// Property filters that select data for this aggregation.
    #[serde(rename = "property_filters", default, with = "::serde_with::rust::double_option")]
    pub property_filters: Option<Option<Vec<Vec<crate::datadogV2::model::ExperimentsWarehouseFilterInput>>>>,
    /// Calculation used to evaluate the threshold.
    #[serde(rename = "threshold_aggregation_type")]
    pub threshold_aggregation_type: Option<String>,
    /// Value used to determine whether the threshold is breached.
    #[serde(rename = "threshold_breach_value")]
    pub threshold_breach_value: Option<f64>,
    /// Operator used to compare the calculated value with the threshold.
    #[serde(rename = "threshold_comparison_operator")]
    pub threshold_comparison_operator: Option<String>,
    /// Time unit for the threshold evaluation window. Supports seconds, minutes, hours, days, calendar_days, and
/// weeks. Calendar days start at midnight on the assignment day. Other units start at the assignment time.
    #[serde(rename = "threshold_timeframe_dimension")]
    pub threshold_timeframe_dimension: Option<String>,
    /// End of the threshold evaluation window, measured from assignment in the configured time unit.
    #[serde(rename = "threshold_timeframe_value")]
    pub threshold_timeframe_value: Option<f64>,
    /// End offset of the aggregation window from assignment, in timeframe_unit.
    #[serde(rename = "timeframe_end_value")]
    pub timeframe_end_value: Option<f64>,
    /// Start offset of the aggregation window from assignment, in timeframe_unit.
    #[serde(rename = "timeframe_start_value")]
    pub timeframe_start_value: Option<f64>,
    /// Time unit for the aggregation window. Calendar days are measured from midnight on the assignment day.
/// Other units are measured from the assignment time.
    #[serde(rename = "timeframe_unit")]
    pub timeframe_unit: Option<String>,
    /// Reference to a measure defined in a warehouse metric SQL model.
    #[serde(rename = "warehouse_metric_measure")]
    pub warehouse_metric_measure: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregationWarehouseMetricMeasure,
    /// Fixed lower bound used to cap extreme measure values.
    #[serde(rename = "winsor_lower_fixed_value")]
    pub winsor_lower_fixed_value: Option<f64>,
    /// Percentile used to determine the lower bound for extreme measure values.
    #[serde(rename = "winsor_lower_percentile")]
    pub winsor_lower_percentile: Option<f64>,
    /// Fixed upper bound used to cap extreme measure values.
    #[serde(rename = "winsor_upper_fixed_value")]
    pub winsor_upper_fixed_value: Option<f64>,
    /// Percentile used to determine the upper bound for extreme measure values.
    #[serde(rename = "winsor_upper_percentile")]
    pub winsor_upper_percentile: Option<f64>,
    /// Method used to cap extreme measure values before aggregation.
    #[serde(rename = "winsorization_strategy")]
    pub winsorization_strategy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsWarehouseMetricAggregationInput {
    pub fn new(
        operation: String,
        warehouse_metric_measure: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregationWarehouseMetricMeasure,
    ) -> ExperimentsWarehouseMetricAggregationInput {
        ExperimentsWarehouseMetricAggregationInput {
            aging_threshold_days: None,
            datadog_metric_measure: None,
            enable_aging_subject_filter: None,
            operation,
            property_filters: None,
            threshold_aggregation_type: None,
            threshold_breach_value: None,
            threshold_comparison_operator: None,
            threshold_timeframe_dimension: None,
            threshold_timeframe_value: None,
            timeframe_end_value: None,
            timeframe_start_value: None,
            timeframe_unit: None,
            warehouse_metric_measure,
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
        value: Option<crate::datadogV2::model::ExperimentsNullableDatadogMetricMeasureInput>,
    ) -> Self {
        self.datadog_metric_measure = Some(value);
        self
    }

    pub fn enable_aging_subject_filter(mut self, value: bool) -> Self {
        self.enable_aging_subject_filter = Some(value);
        self
    }

    pub fn property_filters(
        mut self,
        value: Option<Vec<Vec<crate::datadogV2::model::ExperimentsWarehouseFilterInput>>>,
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

impl<'de> Deserialize<'de> for ExperimentsWarehouseMetricAggregationInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsWarehouseMetricAggregationInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsWarehouseMetricAggregationInputVisitor {
            type Value = ExperimentsWarehouseMetricAggregationInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aging_threshold_days: Option<i64> = None;
                let mut datadog_metric_measure: Option<
                    Option<crate::datadogV2::model::ExperimentsNullableDatadogMetricMeasureInput>,
                > = None;
                let mut enable_aging_subject_filter: Option<bool> = None;
                let mut operation: Option<String> = None;
                let mut property_filters: Option<
                    Option<Vec<Vec<crate::datadogV2::model::ExperimentsWarehouseFilterInput>>>,
                > = None;
                let mut threshold_aggregation_type: Option<String> = None;
                let mut threshold_breach_value: Option<f64> = None;
                let mut threshold_comparison_operator: Option<String> = None;
                let mut threshold_timeframe_dimension: Option<String> = None;
                let mut threshold_timeframe_value: Option<f64> = None;
                let mut timeframe_end_value: Option<f64> = None;
                let mut timeframe_start_value: Option<f64> = None;
                let mut timeframe_unit: Option<String> = None;
                let mut warehouse_metric_measure: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregationWarehouseMetricMeasure> = None;
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
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "property_filters" => {
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
                let operation = operation.ok_or_else(|| M::Error::missing_field("operation"))?;
                let warehouse_metric_measure = warehouse_metric_measure
                    .ok_or_else(|| M::Error::missing_field("warehouse_metric_measure"))?;

                let content = ExperimentsWarehouseMetricAggregationInput {
                    aging_threshold_days,
                    datadog_metric_measure,
                    enable_aging_subject_filter,
                    operation,
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

        deserializer.deserialize_any(ExperimentsWarehouseMetricAggregationInputVisitor)
    }
}
