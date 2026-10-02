// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fields supplied to update the metric. Every attribute is optional; omit an attribute to leave it unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsUpdateMetricV2RequestDataAttributes {
    /// Source of the data backing this metric.
    #[serde(rename = "data_source_type")]
    pub data_source_type: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType>,
    /// Measure and calculation settings for a numerator or denominator aggregation. Supply exactly one non-null measure.
    #[serde(rename = "denominator_aggregation")]
    pub denominator_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation>,
    /// Send null to clear the description. Omit to leave it unchanged.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Direction of change that represents an improvement for this metric.
    #[serde(rename = "desired_change")]
    pub desired_change: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange>,
    /// Whether results render as a percentage. Omit to leave it unchanged.
    #[serde(rename = "format_as_percent")]
    pub format_as_percent: Option<bool>,
    /// Send null to clear a stored threshold. Omit to leave it unchanged.
    #[serde(rename = "guardrail_cutoff_threshold", default, with = "::serde_with::rust::double_option")]
    pub guardrail_cutoff_threshold: Option<Option<f64>>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Name of the metric. Omit to leave it unchanged.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Measure and calculation settings for a numerator or denominator aggregation. Supply exactly one non-null measure.
    #[serde(rename = "numerator_aggregation")]
    pub numerator_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation>,
    /// Measure and percentile to calculate for the metric. Supply exactly one non-null measure.
    #[serde(rename = "percentile_aggregation")]
    pub percentile_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsUpdateMetricV2RequestDataAttributes {
    pub fn new() -> ExperimentsUpdateMetricV2RequestDataAttributes {
        ExperimentsUpdateMetricV2RequestDataAttributes {
            data_source_type: None,
            denominator_aggregation: None,
            description: None,
            desired_change: None,
            format_as_percent: None,
            guardrail_cutoff_threshold: None,
            migration_metadata: None,
            name: None,
            numerator_aggregation: None,
            percentile_aggregation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_source_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType,
    ) -> Self {
        self.data_source_type = Some(value);
        self
    }

    pub fn denominator_aggregation(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation,
    ) -> Self {
        self.denominator_aggregation = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn desired_change(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange,
    ) -> Self {
        self.desired_change = Some(value);
        self
    }

    pub fn format_as_percent(mut self, value: bool) -> Self {
        self.format_as_percent = Some(value);
        self
    }

    pub fn guardrail_cutoff_threshold(mut self, value: Option<f64>) -> Self {
        self.guardrail_cutoff_threshold = Some(value);
        self
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn numerator_aggregation(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation,
    ) -> Self {
        self.numerator_aggregation = Some(value);
        self
    }

    pub fn percentile_aggregation(
        mut self,
        value: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation,
    ) -> Self {
        self.percentile_aggregation = Some(value);
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

impl Default for ExperimentsUpdateMetricV2RequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsUpdateMetricV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsUpdateMetricV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsUpdateMetricV2RequestDataAttributesVisitor {
            type Value = ExperimentsUpdateMetricV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source_type: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType> = None;
                let mut denominator_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation> = None;
                let mut description: Option<Option<String>> = None;
                let mut desired_change: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange> = None;
                let mut format_as_percent: Option<bool> = None;
                let mut guardrail_cutoff_threshold: Option<Option<f64>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut numerator_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation> = None;
                let mut percentile_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source_type) = data_source_type {
                                match _data_source_type {
                                    crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType::UnparsedObject(_data_source_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "denominator_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            denominator_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _denominator_aggregation) = denominator_aggregation {
                                match _denominator_aggregation {
                                    crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation::UnparsedObject(_denominator_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
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
                                    crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange::UnparsedObject(_desired_change) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "format_as_percent" => {
                            if v.is_null() {
                                continue;
                            }
                            format_as_percent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_cutoff_threshold" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            guardrail_cutoff_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            numerator_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _numerator_aggregation) = numerator_aggregation {
                                match _numerator_aggregation {
                                    crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation::UnparsedObject(_numerator_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "percentile_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            percentile_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _percentile_aggregation) = percentile_aggregation {
                                match _percentile_aggregation {
                                    crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation::UnparsedObject(_percentile_aggregation) => {
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

                let content = ExperimentsUpdateMetricV2RequestDataAttributes {
                    data_source_type,
                    denominator_aggregation,
                    description,
                    desired_change,
                    format_as_percent,
                    guardrail_cutoff_threshold,
                    migration_metadata,
                    name,
                    numerator_aggregation,
                    percentile_aggregation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsUpdateMetricV2RequestDataAttributesVisitor)
    }
}
