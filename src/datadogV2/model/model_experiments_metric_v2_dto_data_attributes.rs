// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMetricV2DTODataAttributes {
    /// Time when this resource was certified.
    #[serde(
        rename = "certified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub certified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time when this resource was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Source of the data used to calculate the metric.
    #[serde(rename = "data_source_type")]
    pub data_source_type:
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDataSourceType>,
    /// Source measure and aggregation settings for a metric value.
    #[serde(
        rename = "denominator_aggregation",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub denominator_aggregation: Option<
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation>,
    >,
    /// Text that explains the metric.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Direction of metric change considered desirable.
    #[serde(rename = "desired_change")]
    pub desired_change:
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange>,
    /// Number of experiments that reference this resource.
    #[serde(
        rename = "experiment_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_count: Option<Option<i64>>,
    /// Whether to display the metric value as a percentage.
    #[serde(rename = "format_as_percent")]
    pub format_as_percent: Option<bool>,
    /// Threshold used when evaluating this metric as a guardrail.
    #[serde(
        rename = "guardrail_cutoff_threshold",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub guardrail_cutoff_threshold: Option<Option<f64>>,
    /// Type of metric calculation.
    #[serde(rename = "metric_type")]
    pub metric_type:
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesMetricType>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the metric.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Source measure and aggregation settings for a metric value.
    #[serde(
        rename = "numerator_aggregation",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub numerator_aggregation: Option<
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation>,
    >,
    /// Source measure and settings for a percentile metric.
    #[serde(
        rename = "percentile_aggregation",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub percentile_aggregation: Option<
        Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregation>,
    >,
    /// URL with supporting information about the metric.
    #[serde(rename = "reference_url")]
    pub reference_url: Option<String>,
    /// Time when this resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMetricV2DTODataAttributes {
    pub fn new() -> ExperimentsMetricV2DTODataAttributes {
        ExperimentsMetricV2DTODataAttributes {
            certified_at: None,
            created_at: None,
            data_source_type: None,
            denominator_aggregation: None,
            description: None,
            desired_change: None,
            experiment_count: None,
            format_as_percent: None,
            guardrail_cutoff_threshold: None,
            metric_type: None,
            migration_metadata: None,
            name: None,
            numerator_aggregation: None,
            percentile_aggregation: None,
            reference_url: None,
            updated_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn certified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.certified_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn data_source_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDataSourceType,
    ) -> Self {
        self.data_source_type = Some(value);
        self
    }

    pub fn denominator_aggregation(
        mut self,
        value: Option<
            crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation,
        >,
    ) -> Self {
        self.denominator_aggregation = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn desired_change(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange,
    ) -> Self {
        self.desired_change = Some(value);
        self
    }

    pub fn experiment_count(mut self, value: Option<i64>) -> Self {
        self.experiment_count = Some(value);
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

    pub fn metric_type(
        mut self,
        value: crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesMetricType,
    ) -> Self {
        self.metric_type = Some(value);
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
        value: Option<
            crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation,
        >,
    ) -> Self {
        self.numerator_aggregation = Some(value);
        self
    }

    pub fn percentile_aggregation(
        mut self,
        value: Option<
            crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregation,
        >,
    ) -> Self {
        self.percentile_aggregation = Some(value);
        self
    }

    pub fn reference_url(mut self, value: String) -> Self {
        self.reference_url = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl Default for ExperimentsMetricV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsMetricV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMetricV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsMetricV2DTODataAttributesVisitor {
            type Value = ExperimentsMetricV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut certified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut data_source_type: Option<
                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDataSourceType,
                > = None;
                let mut denominator_aggregation: Option<Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation>> = None;
                let mut description: Option<String> = None;
                let mut desired_change: Option<
                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange,
                > = None;
                let mut experiment_count: Option<Option<i64>> = None;
                let mut format_as_percent: Option<bool> = None;
                let mut guardrail_cutoff_threshold: Option<Option<f64>> = None;
                let mut metric_type: Option<
                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesMetricType,
                > = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut numerator_aggregation: Option<Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesNumeratorAggregation>> = None;
                let mut percentile_aggregation: Option<Option<crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesPercentileAggregation>> = None;
                let mut reference_url: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "certified_at" => {
                            certified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source_type) = data_source_type {
                                match _data_source_type {
                                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDataSourceType::UnparsedObject(_data_source_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "denominator_aggregation" => {
                            denominator_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
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
                                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesDesiredChange::UnparsedObject(_desired_change) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "experiment_count" => {
                            experiment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "metric_type" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _metric_type) = metric_type {
                                match _metric_type {
                                    crate::datadogV2::model::ExperimentsMetricV2DTODataAttributesMetricType::UnparsedObject(_metric_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                            numerator_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentile_aggregation" => {
                            percentile_aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_url" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsMetricV2DTODataAttributes {
                    certified_at,
                    created_at,
                    data_source_type,
                    denominator_aggregation,
                    description,
                    desired_change,
                    experiment_count,
                    format_as_percent,
                    guardrail_cutoff_threshold,
                    metric_type,
                    migration_metadata,
                    name,
                    numerator_aggregation,
                    percentile_aggregation,
                    reference_url,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsMetricV2DTODataAttributesVisitor)
    }
}
