// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// One statistical comparison for a metric and variant.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
    /// Lower and upper bounds of the reported statistical interval.
    #[serde(rename = "confidence_interval")]
    pub confidence_interval: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsConfidenceInterval>,
    /// Configured nominal confidence level. Interval bounds can use an adjusted level for multiple testing or hybrid methods.
    #[serde(rename = "confidence_level")]
    pub confidence_level: Option<f64>,
    /// Expected reduction in minimum expected regret from collecting more sample data.
    #[serde(rename = "evsi")]
    pub evsi: Option<f64>,
    /// Expected effect when the effect is positive.
    #[serde(rename = "expectation_above_zero")]
    pub expectation_above_zero: Option<f64>,
    /// Expected effect when the effect is negative.
    #[serde(rename = "expectation_below_zero")]
    pub expectation_below_zero: Option<f64>,
    /// Estimated lift across the population. Calculated as metric coverage multiplied by the experiment lift.
    #[serde(rename = "global_lift")]
    pub global_lift: Option<f64>,
    /// Lower bound of the estimated lift across the population.
    #[serde(rename = "global_lift_lower_bound")]
    pub global_lift_lower_bound: Option<f64>,
    /// Upper bound of the estimated lift across the population.
    #[serde(rename = "global_lift_upper_bound")]
    pub global_lift_upper_bound: Option<f64>,
    /// Whether CUPED used pre-experiment data to reduce variance in this result.
    #[serde(rename = "is_cuped_adjusted")]
    pub is_cuped_adjusted: Option<bool>,
    /// Whether the statistical result is marked as unreliable.
    #[serde(rename = "is_unreliable")]
    pub is_unreliable: Option<bool>,
    /// Whether the reported lift is relative or absolute.
    #[serde(rename = "lift_type")]
    pub lift_type: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsLiftType>,
    /// Statistical method used to calculate this result.
    #[serde(rename = "method")]
    pub method: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod>,
    /// The smaller expected opportunity cost of choosing treatment or control.
    #[serde(rename = "minimum_expected_regret")]
    pub minimum_expected_regret: Option<f64>,
    /// Probability, under the no-effect hypothesis, of a result at least as extreme as the observed result.
    #[serde(rename = "p_value")]
    pub p_value: Option<f64>,
    /// Estimated difference between the variant and control for this metric.
    #[serde(rename = "point_estimate")]
    pub point_estimate: Option<f64>,
    /// Estimated probability that the effect is greater than zero.
    #[serde(rename = "probability_above_zero")]
    pub probability_above_zero: Option<f64>,
    /// Estimated probability that the effect is less than zero.
    #[serde(rename = "probability_below_zero")]
    pub probability_below_zero: Option<f64>,
    /// Estimated uncertainty in the effect estimate.
    #[serde(rename = "standard_error")]
    pub standard_error: Option<f64>,
    /// Reason that the statistical result is marked as unreliable.
    #[serde(rename = "unreliable_reason")]
    pub unreliable_reason: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason>,
    /// Metric value calculated for this variant.
    #[serde(rename = "variant_metric_value", default, with = "::serde_with::rust::double_option")]
    pub variant_metric_value: Option<Option<f64>>,
    /// Standardized statistic used to compare the observed effect with zero.
    #[serde(rename = "z_score")]
    pub z_score: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
    pub fn new() -> ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
        ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
            confidence_interval: None,
            confidence_level: None,
            evsi: None,
            expectation_above_zero: None,
            expectation_below_zero: None,
            global_lift: None,
            global_lift_lower_bound: None,
            global_lift_upper_bound: None,
            is_cuped_adjusted: None,
            is_unreliable: None,
            lift_type: None,
            method: None,
            minimum_expected_regret: None,
            p_value: None,
            point_estimate: None,
            probability_above_zero: None,
            probability_below_zero: None,
            standard_error: None,
            unreliable_reason: None,
            variant_metric_value: None,
            z_score: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confidence_interval(
        mut self,
        value: crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsConfidenceInterval,
    ) -> Self {
        self.confidence_interval = Some(value);
        self
    }

    pub fn confidence_level(mut self, value: f64) -> Self {
        self.confidence_level = Some(value);
        self
    }

    pub fn evsi(mut self, value: f64) -> Self {
        self.evsi = Some(value);
        self
    }

    pub fn expectation_above_zero(mut self, value: f64) -> Self {
        self.expectation_above_zero = Some(value);
        self
    }

    pub fn expectation_below_zero(mut self, value: f64) -> Self {
        self.expectation_below_zero = Some(value);
        self
    }

    pub fn global_lift(mut self, value: f64) -> Self {
        self.global_lift = Some(value);
        self
    }

    pub fn global_lift_lower_bound(mut self, value: f64) -> Self {
        self.global_lift_lower_bound = Some(value);
        self
    }

    pub fn global_lift_upper_bound(mut self, value: f64) -> Self {
        self.global_lift_upper_bound = Some(value);
        self
    }

    pub fn is_cuped_adjusted(mut self, value: bool) -> Self {
        self.is_cuped_adjusted = Some(value);
        self
    }

    pub fn is_unreliable(mut self, value: bool) -> Self {
        self.is_unreliable = Some(value);
        self
    }

    pub fn lift_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsLiftType,
    ) -> Self {
        self.lift_type = Some(value);
        self
    }

    pub fn method(
        mut self,
        value: crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod,
    ) -> Self {
        self.method = Some(value);
        self
    }

    pub fn minimum_expected_regret(mut self, value: f64) -> Self {
        self.minimum_expected_regret = Some(value);
        self
    }

    pub fn p_value(mut self, value: f64) -> Self {
        self.p_value = Some(value);
        self
    }

    pub fn point_estimate(mut self, value: f64) -> Self {
        self.point_estimate = Some(value);
        self
    }

    pub fn probability_above_zero(mut self, value: f64) -> Self {
        self.probability_above_zero = Some(value);
        self
    }

    pub fn probability_below_zero(mut self, value: f64) -> Self {
        self.probability_below_zero = Some(value);
        self
    }

    pub fn standard_error(mut self, value: f64) -> Self {
        self.standard_error = Some(value);
        self
    }

    pub fn unreliable_reason(
        mut self,
        value: crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason,
    ) -> Self {
        self.unreliable_reason = Some(value);
        self
    }

    pub fn variant_metric_value(mut self, value: Option<f64>) -> Self {
        self.variant_metric_value = Some(value);
        self
    }

    pub fn z_score(mut self, value: f64) -> Self {
        self.z_score = Some(value);
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

impl Default for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsVisitor
        {
            type Value = ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut confidence_interval: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsConfidenceInterval> = None;
                let mut confidence_level: Option<f64> = None;
                let mut evsi: Option<f64> = None;
                let mut expectation_above_zero: Option<f64> = None;
                let mut expectation_below_zero: Option<f64> = None;
                let mut global_lift: Option<f64> = None;
                let mut global_lift_lower_bound: Option<f64> = None;
                let mut global_lift_upper_bound: Option<f64> = None;
                let mut is_cuped_adjusted: Option<bool> = None;
                let mut is_unreliable: Option<bool> = None;
                let mut lift_type: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsLiftType> = None;
                let mut method: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod> = None;
                let mut minimum_expected_regret: Option<f64> = None;
                let mut p_value: Option<f64> = None;
                let mut point_estimate: Option<f64> = None;
                let mut probability_above_zero: Option<f64> = None;
                let mut probability_below_zero: Option<f64> = None;
                let mut standard_error: Option<f64> = None;
                let mut unreliable_reason: Option<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason> = None;
                let mut variant_metric_value: Option<Option<f64>> = None;
                let mut z_score: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "confidence_interval" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_interval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_level" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evsi" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            evsi = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expectation_above_zero" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            expectation_above_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expectation_below_zero" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            expectation_below_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_lift" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_lift =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_lift_lower_bound" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_lift_lower_bound =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_lift_upper_bound" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_lift_upper_bound =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_cuped_adjusted" => {
                            if v.is_null() {
                                continue;
                            }
                            is_cuped_adjusted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_unreliable" => {
                            if v.is_null() {
                                continue;
                            }
                            is_unreliable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lift_type" => {
                            if v.is_null() {
                                continue;
                            }
                            lift_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _lift_type) = lift_type {
                                match _lift_type {
                                    crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsLiftType::UnparsedObject(_lift_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "method" => {
                            if v.is_null() {
                                continue;
                            }
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _method) = method {
                                match _method {
                                    crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsMethod::UnparsedObject(_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "minimum_expected_regret" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            minimum_expected_regret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "p_value" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            p_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "point_estimate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            point_estimate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "probability_above_zero" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            probability_above_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "probability_below_zero" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            probability_below_zero =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "standard_error" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            standard_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unreliable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            unreliable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unreliable_reason) = unreliable_reason {
                                match _unreliable_reason {
                                    crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsUnreliableReason::UnparsedObject(_unreliable_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "variant_metric_value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            variant_metric_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "z_score" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            z_score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItems {
                        confidence_interval,
                        confidence_level,
                        evsi,
                        expectation_above_zero,
                        expectation_below_zero,
                        global_lift,
                        global_lift_lower_bound,
                        global_lift_upper_bound,
                        is_cuped_adjusted,
                        is_unreliable,
                        lift_type,
                        method,
                        minimum_expected_regret,
                        p_value,
                        point_estimate,
                        probability_above_zero,
                        probability_below_zero,
                        standard_error,
                        unreliable_reason,
                        variant_metric_value,
                        z_score,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsVariantResultsV2DTODataAttributesMetricsItemsAnalysesItemsVisitor,
        )
    }
}
