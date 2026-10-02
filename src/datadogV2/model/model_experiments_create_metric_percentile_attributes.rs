// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a percentile metric. Omit numerator_aggregation and denominator_aggregation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateMetricPercentileAttributes {
    /// Source of the data backing this metric.
    #[serde(rename = "data_source_type")]
    pub data_source_type: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType,
    /// Description of the metric. Send null to leave it unset.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    /// Direction of change that represents an improvement for this metric.
    #[serde(rename = "desired_change")]
    pub desired_change: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange,
    /// Whether results render as a percentage. Defaults to false when omitted.
    #[serde(rename = "format_as_percent")]
    pub format_as_percent: Option<bool>,
    /// Guardrail cutoff threshold. Send null to leave it unset.
    #[serde(rename = "guardrail_cutoff_threshold", default, with = "::serde_with::rust::double_option")]
    pub guardrail_cutoff_threshold: Option<Option<f64>>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Name of the metric.
    #[serde(rename = "name")]
    pub name: String,
    /// Measure and percentile to calculate for the metric. Supply exactly one non-null measure.
    #[serde(rename = "percentile_aggregation")]
    pub percentile_aggregation: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsCreateMetricPercentileAttributes {
    pub fn new(
        data_source_type: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType,
        desired_change: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange,
        name: String,
        percentile_aggregation: crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation,
    ) -> ExperimentsCreateMetricPercentileAttributes {
        ExperimentsCreateMetricPercentileAttributes {
            data_source_type,
            description: None,
            desired_change,
            format_as_percent: None,
            guardrail_cutoff_threshold: None,
            migration_metadata: None,
            name,
            percentile_aggregation,
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for ExperimentsCreateMetricPercentileAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateMetricPercentileAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsCreateMetricPercentileAttributesVisitor {
            type Value = ExperimentsCreateMetricPercentileAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source_type: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType> = None;
                let mut description: Option<Option<String>> = None;
                let mut desired_change: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange> = None;
                let mut format_as_percent: Option<bool> = None;
                let mut guardrail_cutoff_threshold: Option<Option<f64>> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut percentile_aggregation: Option<crate::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation> = None;
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
                        "percentile_aggregation" => {
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
                let percentile_aggregation = percentile_aggregation
                    .ok_or_else(|| M::Error::missing_field("percentile_aggregation"))?;

                let content = ExperimentsCreateMetricPercentileAttributes {
                    data_source_type,
                    description,
                    desired_change,
                    format_as_percent,
                    guardrail_cutoff_threshold,
                    migration_metadata,
                    name,
                    percentile_aggregation,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsCreateMetricPercentileAttributesVisitor)
    }
}
