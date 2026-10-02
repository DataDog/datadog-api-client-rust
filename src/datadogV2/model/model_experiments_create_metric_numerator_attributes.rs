// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a metric calculated from a numerator and an optional denominator. Omit percentile_aggregation. Omit denominator_aggregation when unused.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateMetricNumeratorAttributes {
    /// Source of the data backing this metric.
    #[serde(rename = "data_source_type")]
    pub data_source_type:
        crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType,
    /// Measure and calculation settings for a numerator or denominator aggregation. Supply exactly one non-null measure.
    #[serde(rename = "denominator_aggregation")]
    pub denominator_aggregation: Option<
        crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation,
    >,
    /// Description of the metric. Send null to leave it unset.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// Direction of change that represents an improvement for this metric.
    #[serde(rename = "desired_change")]
    pub desired_change:
        crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange,
    /// Whether results render as a percentage. Defaults to false when omitted.
    #[serde(rename = "format_as_percent")]
    pub format_as_percent: Option<bool>,
    /// Guardrail cutoff threshold. Send null to leave it unset.
    #[serde(
        rename = "guardrail_cutoff_threshold",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub guardrail_cutoff_threshold: Option<Option<f64>>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Name of the metric.
    #[serde(rename = "name")]
    pub name: String,
    /// Measure and calculation settings for a numerator or denominator aggregation. Supply exactly one non-null measure.
    #[serde(rename = "numerator_aggregation")]
    pub numerator_aggregation:
        crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateMetricNumeratorAttributes {
    pub fn new(
        data_source_type: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType,
        desired_change: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange,
        name: String,
        numerator_aggregation: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation,
    ) -> ExperimentsCreateMetricNumeratorAttributes {
        ExperimentsCreateMetricNumeratorAttributes {
            data_source_type,
            denominator_aggregation: None,
            description: None,
            desired_change,
            format_as_percent: None,
            guardrail_cutoff_threshold: None,
            migration_metadata: None,
            name,
            numerator_aggregation,
            _unparsed: false,
        }
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
}

impl<'de> Deserialize<'de> for ExperimentsCreateMetricNumeratorAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateMetricNumeratorAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsCreateMetricNumeratorAttributesVisitor {
            type Value = ExperimentsCreateMetricNumeratorAttributes;

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
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source_type" => {
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator_aggregation" => {
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
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source_type =
                    data_source_type.ok_or_else(|| M::Error::missing_field("data_source_type"))?;
                let desired_change =
                    desired_change.ok_or_else(|| M::Error::missing_field("desired_change"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let numerator_aggregation = numerator_aggregation
                    .ok_or_else(|| M::Error::missing_field("numerator_aggregation"))?;

                let content = ExperimentsCreateMetricNumeratorAttributes {
                    data_source_type,
                    denominator_aggregation,
                    description,
                    desired_change,
                    format_as_percent,
                    guardrail_cutoff_threshold,
                    migration_metadata,
                    name,
                    numerator_aggregation,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsCreateMetricNumeratorAttributesVisitor)
    }
}
