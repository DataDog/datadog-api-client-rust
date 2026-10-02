// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metric values and statistical results for one variant.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
    /// Statistical analyses calculated for this metric and variant.
    #[serde(rename = "analyses")]
    pub analyses: Option<Vec<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems>>,
    /// Number of subjects assigned to this variant.
    #[serde(rename = "assignment_count")]
    pub assignment_count: Option<i64>,
    /// Estimated share of the global metric total from the eligible population if that population received
/// control. The estimate can exceed 1.
    #[serde(rename = "coverage")]
    pub coverage: Option<f64>,
    /// Population totals and allocation used to calculate metric coverage.
    #[serde(rename = "coverage_summary")]
    pub coverage_summary: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary>,
    /// Reason that metric coverage could not be calculated.
    #[serde(rename = "coverage_unavailable_reason")]
    pub coverage_unavailable_reason: Option<String>,
    /// Aggregated denominator value for this metric and variant.
    #[serde(rename = "denominator", default, with = "::serde_with::rust::double_option")]
    pub denominator: Option<Option<f64>>,
    /// Direction of metric change considered desirable.
    #[serde(rename = "desired_change")]
    pub desired_change: Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange>,
    /// ID of the metric represented by this entry.
    #[serde(rename = "metric_id")]
    pub metric_id: Option<String>,
    /// Display name of the metric represented by this entry.
    #[serde(rename = "metric_name")]
    pub metric_name: Option<String>,
    /// Aggregated numerator value for this metric and variant.
    #[serde(rename = "numerator", default, with = "::serde_with::rust::double_option")]
    pub numerator: Option<Option<f64>>,
    /// Name of the property used to split this metric result.
    #[serde(rename = "sub_metric_property_name")]
    pub sub_metric_property_name: Option<String>,
    /// Property value represented by this split metric result.
    #[serde(rename = "sub_metric_property_value")]
    pub sub_metric_property_value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
    pub fn new() -> ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
        ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
            analyses: None,
            assignment_count: None,
            coverage: None,
            coverage_summary: None,
            coverage_unavailable_reason: None,
            denominator: None,
            desired_change: None,
            metric_id: None,
            metric_name: None,
            numerator: None,
            sub_metric_property_name: None,
            sub_metric_property_value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analyses(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems>,
    ) -> Self {
        self.analyses = Some(value);
        self
    }

    pub fn assignment_count(mut self, value: i64) -> Self {
        self.assignment_count = Some(value);
        self
    }

    pub fn coverage(mut self, value: f64) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn coverage_summary(
        mut self,
        value: crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary,
    ) -> Self {
        self.coverage_summary = Some(value);
        self
    }

    pub fn coverage_unavailable_reason(mut self, value: String) -> Self {
        self.coverage_unavailable_reason = Some(value);
        self
    }

    pub fn denominator(mut self, value: Option<f64>) -> Self {
        self.denominator = Some(value);
        self
    }

    pub fn desired_change(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange,
    ) -> Self {
        self.desired_change = Some(value);
        self
    }

    pub fn metric_id(mut self, value: String) -> Self {
        self.metric_id = Some(value);
        self
    }

    pub fn metric_name(mut self, value: String) -> Self {
        self.metric_name = Some(value);
        self
    }

    pub fn numerator(mut self, value: Option<f64>) -> Self {
        self.numerator = Some(value);
        self
    }

    pub fn sub_metric_property_name(mut self, value: String) -> Self {
        self.sub_metric_property_name = Some(value);
        self
    }

    pub fn sub_metric_property_value(mut self, value: String) -> Self {
        self.sub_metric_property_value = Some(value);
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

impl Default for ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsVariantResultsV2DTODataAttributesMetricsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsVisitor {
            type Value = ExperimentsVariantResultsV2DTODataAttributesMetricsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analyses: Option<Vec<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems>> = None;
                let mut assignment_count: Option<i64> = None;
                let mut coverage: Option<f64> = None;
                let mut coverage_summary: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary> = None;
                let mut coverage_unavailable_reason: Option<String> = None;
                let mut denominator: Option<Option<f64>> = None;
                let mut desired_change: Option<
                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange,
                > = None;
                let mut metric_id: Option<String> = None;
                let mut metric_name: Option<String> = None;
                let mut numerator: Option<Option<f64>> = None;
                let mut sub_metric_property_name: Option<String> = None;
                let mut sub_metric_property_value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analyses" => {
                            if v.is_null() {
                                continue;
                            }
                            analyses = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignment_count" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            coverage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage_summary" => {
                            if v.is_null() {
                                continue;
                            }
                            coverage_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage_unavailable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            coverage_unavailable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "denominator" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "desired_change" => {
                            if v.is_null() {
                                continue;
                            }
                            desired_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _desired_change) = desired_change {
                                match _desired_change {
                                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange::UnparsedObject(_desired_change) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metric_id" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_name" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_metric_property_name" => {
                            if v.is_null() {
                                continue;
                            }
                            sub_metric_property_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_metric_property_value" => {
                            if v.is_null() {
                                continue;
                            }
                            sub_metric_property_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsVariantResultsV2DTODataAttributesMetricsItems {
                    analyses,
                    assignment_count,
                    coverage,
                    coverage_summary,
                    coverage_unavailable_reason,
                    denominator,
                    desired_change,
                    metric_id,
                    metric_name,
                    numerator,
                    sub_metric_property_name,
                    sub_metric_property_value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsVariantResultsV2DTODataAttributesMetricsItemsVisitor)
    }
}
