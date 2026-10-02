// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Population totals and allocation used to calculate metric coverage.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
    /// Total metric value for the control population in the coverage calculation.
    #[serde(rename = "control_total")]
    pub control_total: Option<f64>,
    /// Estimated share of the global metric total from the eligible population if that population received
    /// control. The estimate can exceed 1.
    #[serde(rename = "coverage")]
    pub coverage: Option<f64>,
    /// Reason that metric coverage could not be calculated.
    #[serde(rename = "coverage_unavailable_reason")]
    pub coverage_unavailable_reason: Option<String>,
    /// Estimated metric total for the eligible population if that population received control.
    #[serde(rename = "eligible_population_total")]
    pub eligible_population_total: Option<f64>,
    /// Total metric value for the experiment population in the coverage calculation.
    #[serde(rename = "experiment_total")]
    pub experiment_total: Option<f64>,
    /// Observed metric total across subjects inside and outside the experiment.
    #[serde(rename = "global_metric_total")]
    pub global_metric_total: Option<f64>,
    /// Fraction of eligible traffic allocated to the experiment, weighted by time.
    #[serde(rename = "traffic_allocation")]
    pub traffic_allocation: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
    pub fn new() -> ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
        ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
            control_total: None,
            coverage: None,
            coverage_unavailable_reason: None,
            eligible_population_total: None,
            experiment_total: None,
            global_metric_total: None,
            traffic_allocation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn control_total(mut self, value: f64) -> Self {
        self.control_total = Some(value);
        self
    }

    pub fn coverage(mut self, value: f64) -> Self {
        self.coverage = Some(value);
        self
    }

    pub fn coverage_unavailable_reason(mut self, value: String) -> Self {
        self.coverage_unavailable_reason = Some(value);
        self
    }

    pub fn eligible_population_total(mut self, value: f64) -> Self {
        self.eligible_population_total = Some(value);
        self
    }

    pub fn experiment_total(mut self, value: f64) -> Self {
        self.experiment_total = Some(value);
        self
    }

    pub fn global_metric_total(mut self, value: f64) -> Self {
        self.global_metric_total = Some(value);
        self
    }

    pub fn traffic_allocation(mut self, value: f64) -> Self {
        self.traffic_allocation = Some(value);
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

impl Default for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummaryVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummaryVisitor
        {
            type Value = ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut control_total: Option<f64> = None;
                let mut coverage: Option<f64> = None;
                let mut coverage_unavailable_reason: Option<String> = None;
                let mut eligible_population_total: Option<f64> = None;
                let mut experiment_total: Option<f64> = None;
                let mut global_metric_total: Option<f64> = None;
                let mut traffic_allocation: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "control_total" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            control_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            coverage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "coverage_unavailable_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            coverage_unavailable_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eligible_population_total" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            eligible_population_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_total" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            experiment_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "global_metric_total" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_metric_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traffic_allocation" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            traffic_allocation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummary {
                        control_total,
                        coverage,
                        coverage_unavailable_reason,
                        eligible_population_total,
                        experiment_total,
                        global_metric_total,
                        traffic_allocation,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsVariantResultsV2DTODataAttributesMetricsItemsCoverageSummaryVisitor,
        )
    }
}
