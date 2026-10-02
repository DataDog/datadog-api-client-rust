// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2ResponseDataAttributes {
    /// End of the time window for experiment assignments.
    #[serde(rename = "assignments_end_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the time window for experiment assignments.
    #[serde(rename = "assignments_start_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time when the experiment was concluded.
    #[serde(rename = "concluded_at", default, with = "::serde_with::rust::double_option")]
    pub concluded_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Outcome and supporting text recorded when the experiment is concluded.
    #[serde(rename = "conclusion")]
    pub conclusion: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesConclusion>,
    /// Time when this resource was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Feature flag, environment, and targeting configuration for the experiment.
    #[serde(rename = "datadog_flag_configuration", default, with = "::serde_with::rust::double_option")]
    pub datadog_flag_configuration: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration>>,
    /// Metrics used to decide the experiment outcome.
    #[serde(rename = "decision_metrics")]
    pub decision_metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems>>,
    /// Key of the variant selected in the experiment decision.
    #[serde(rename = "decision_variant_key")]
    pub decision_variant_key: Option<String>,
    /// End of the time window for metric events.
    #[serde(rename = "events_end_date", default, with = "::serde_with::rust::double_option")]
    pub events_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the time window for metric events.
    #[serde(rename = "events_start_date", default, with = "::serde_with::rust::double_option")]
    pub events_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Kind of experiment. STANDARD is an ordinary experiment. Other values, such as CANARY and HOLDOUT, identify experiments owned by another workflow. New kinds may be added; treat unknown values as non-standard.
    #[serde(rename = "experiment_type")]
    pub experiment_type: Option<String>,
    /// Expected effect that the experiment is intended to test.
    #[serde(rename = "hypothesis")]
    pub hypothesis: Option<String>,
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the experiment.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Suffix used to identify the experiment's pipeline output table.
    #[serde(rename = "pipeline_table_suffix")]
    pub pipeline_table_suffix: Option<String>,
    /// ID of the protocol associated with the experiment.
    #[serde(rename = "protocol_id")]
    pub protocol_id: Option<String>,
    /// Links to supporting material for the experiment.
    #[serde(rename = "related_links")]
    pub related_links: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>>,
    /// Time of the most recent update to the experiment's results.
    #[serde(rename = "results_last_updated", default, with = "::serde_with::rust::double_option")]
    pub results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Properties used to split the experiment results into groups.
    #[serde(rename = "split_by_properties")]
    pub split_by_properties: Option<Vec<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems>>,
    /// Current stage in the experiment lifecycle.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesStatus>,
    /// Values of structured metadata fields attached to the experiment.
    #[serde(rename = "structured_metadata")]
    pub structured_metadata: Option<Vec<crate::datadogV2::model::ExperimentsStructuredMetadataResponse>>,
    /// ID of the subject type used by this configuration.
    #[serde(rename = "subject_type_id", default, with = "::serde_with::rust::double_option")]
    pub subject_type_id: Option<Option<String>>,
    /// Summary text recorded for the experiment.
    #[serde(rename = "summary")]
    pub summary: Option<String>,
    /// Tags attached to the experiment.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Teams associated with the experiment.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    /// Traffic exposure fraction or schedule configured for the experiment.
    #[serde(rename = "traffic_exposure")]
    pub traffic_exposure: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure>,
    /// Time when this resource was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Variants configured for the experiment.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesVariantsItems>>,
    /// Warehouse exposure model and settings used to identify experiment assignments.
    #[serde(rename = "warehouse_exposure_configuration", default, with = "::serde_with::rust::double_option")]
    pub warehouse_exposure_configuration: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentV2ResponseDataAttributes {
    pub fn new() -> ExperimentsPatchExperimentV2ResponseDataAttributes {
        ExperimentsPatchExperimentV2ResponseDataAttributes {
            assignments_end_date: None,
            assignments_start_date: None,
            concluded_at: None,
            conclusion: None,
            created_at: None,
            datadog_flag_configuration: None,
            decision_metrics: None,
            decision_variant_key: None,
            events_end_date: None,
            events_start_date: None,
            experiment_type: None,
            hypothesis: None,
            migration_metadata: None,
            name: None,
            pipeline_table_suffix: None,
            protocol_id: None,
            related_links: None,
            results_last_updated: None,
            split_by_properties: None,
            status: None,
            structured_metadata: None,
            subject_type_id: None,
            summary: None,
            tags: None,
            teams: None,
            traffic_exposure: None,
            updated_at: None,
            variants: None,
            warehouse_exposure_configuration: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignments_end_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.assignments_end_date = Some(value);
        self
    }

    pub fn assignments_start_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.assignments_start_date = Some(value);
        self
    }

    pub fn concluded_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.concluded_at = Some(value);
        self
    }

    pub fn conclusion(
        mut self,
        value: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesConclusion,
    ) -> Self {
        self.conclusion = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn datadog_flag_configuration(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration>,
    ) -> Self {
        self.datadog_flag_configuration = Some(value);
        self
    }

    pub fn decision_metrics(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems>,
    ) -> Self {
        self.decision_metrics = Some(value);
        self
    }

    pub fn decision_variant_key(mut self, value: String) -> Self {
        self.decision_variant_key = Some(value);
        self
    }

    pub fn events_end_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.events_end_date = Some(value);
        self
    }

    pub fn events_start_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.events_start_date = Some(value);
        self
    }

    pub fn experiment_type(mut self, value: String) -> Self {
        self.experiment_type = Some(value);
        self
    }

    pub fn hypothesis(mut self, value: String) -> Self {
        self.hypothesis = Some(value);
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

    pub fn pipeline_table_suffix(mut self, value: String) -> Self {
        self.pipeline_table_suffix = Some(value);
        self
    }

    pub fn protocol_id(mut self, value: String) -> Self {
        self.protocol_id = Some(value);
        self
    }

    pub fn related_links(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>,
    ) -> Self {
        self.related_links = Some(value);
        self
    }

    pub fn results_last_updated(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.results_last_updated = Some(value);
        self
    }

    pub fn split_by_properties(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems,
        >,
    ) -> Self {
        self.split_by_properties = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn structured_metadata(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsStructuredMetadataResponse>,
    ) -> Self {
        self.structured_metadata = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: Option<String>) -> Self {
        self.subject_type_id = Some(value);
        self
    }

    pub fn summary(mut self, value: String) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn teams(mut self, value: Vec<String>) -> Self {
        self.teams = Some(value);
        self
    }

    pub fn traffic_exposure(
        mut self,
        value: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure,
    ) -> Self {
        self.traffic_exposure = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn variants(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesVariantsItems>,
    ) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn warehouse_exposure_configuration(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration>,
    ) -> Self {
        self.warehouse_exposure_configuration = Some(value);
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

impl Default for ExperimentsPatchExperimentV2ResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2ResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2ResponseDataAttributesVisitor {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignments_end_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut assignments_start_date: Option<Option<chrono::DateTime<chrono::Utc>>> =
                    None;
                let mut concluded_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut conclusion: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesConclusion> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut datadog_flag_configuration: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration>> = None;
                let mut decision_metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems>> = None;
                let mut decision_variant_key: Option<String> = None;
                let mut events_end_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut events_start_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut experiment_type: Option<String> = None;
                let mut hypothesis: Option<String> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut pipeline_table_suffix: Option<String> = None;
                let mut protocol_id: Option<String> = None;
                let mut related_links: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>> = None;
                let mut results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut split_by_properties: Option<Vec<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems>> = None;
                let mut status: Option<
                    crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesStatus,
                > = None;
                let mut structured_metadata: Option<
                    Vec<crate::datadogV2::model::ExperimentsStructuredMetadataResponse>,
                > = None;
                let mut subject_type_id: Option<Option<String>> = None;
                let mut summary: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut traffic_exposure: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesVariantsItems>> = None;
                let mut warehouse_exposure_configuration: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignments_end_date" => {
                            assignments_end_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignments_start_date" => {
                            assignments_start_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "concluded_at" => {
                            concluded_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conclusion" => {
                            if v.is_null() {
                                continue;
                            }
                            conclusion = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datadog_flag_configuration" => {
                            datadog_flag_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decision_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            decision_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "decision_variant_key" => {
                            if v.is_null() {
                                continue;
                            }
                            decision_variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_end_date" => {
                            events_end_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_start_date" => {
                            events_start_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_type" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hypothesis" => {
                            if v.is_null() {
                                continue;
                            }
                            hypothesis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "pipeline_table_suffix" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_table_suffix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol_id" => {
                            if v.is_null() {
                                continue;
                            }
                            protocol_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "related_links" => {
                            if v.is_null() {
                                continue;
                            }
                            related_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results_last_updated" => {
                            results_last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "split_by_properties" => {
                            if v.is_null() {
                                continue;
                            }
                            split_by_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "structured_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            structured_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            subject_type_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            if v.is_null() {
                                continue;
                            }
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traffic_exposure" => {
                            if v.is_null() {
                                continue;
                            }
                            traffic_exposure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            if v.is_null() {
                                continue;
                            }
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_exposure_configuration" => {
                            warehouse_exposure_configuration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPatchExperimentV2ResponseDataAttributes {
                    assignments_end_date,
                    assignments_start_date,
                    concluded_at,
                    conclusion,
                    created_at,
                    datadog_flag_configuration,
                    decision_metrics,
                    decision_variant_key,
                    events_end_date,
                    events_start_date,
                    experiment_type,
                    hypothesis,
                    migration_metadata,
                    name,
                    pipeline_table_suffix,
                    protocol_id,
                    related_links,
                    results_last_updated,
                    split_by_properties,
                    status,
                    structured_metadata,
                    subject_type_id,
                    summary,
                    tags,
                    teams,
                    traffic_exposure,
                    updated_at,
                    variants,
                    warehouse_exposure_configuration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchExperimentV2ResponseDataAttributesVisitor)
    }
}
