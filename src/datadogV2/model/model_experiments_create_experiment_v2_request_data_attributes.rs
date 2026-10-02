// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration and descriptive fields for the new experiment draft.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributes {
    /// End of the window assignments are read from. Optional; must be after assignments_start_date.
    #[serde(rename = "assignments_end_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the window assignments are read from. Optional.
    #[serde(rename = "assignments_start_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Feature flag, environment, and targeting configuration for a Datadog experiment.
    #[serde(rename = "datadog_flag_configuration", default, with = "::serde_with::rust::double_option")]
    pub datadog_flag_configuration: Option<Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration>>,
    /// Metrics selected to support the experiment decision.
    #[serde(rename = "decision_metrics")]
    pub decision_metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems>>,
    /// End of the window metric events are read from. Optional; must be after events_start_date.
    #[serde(rename = "events_end_date", default, with = "::serde_with::rust::double_option")]
    pub events_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the window metric events are read from. Optional; must fall within the assignments window.
    #[serde(rename = "events_start_date", default, with = "::serde_with::rust::double_option")]
    pub events_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// What the experiment is expected to show. Optional and free-form.
    #[serde(rename = "hypothesis")]
    pub hypothesis: Option<String>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name for the experiment. The only required attribute: a request carrying nothing but a name is accepted.
    #[serde(rename = "name")]
    pub name: String,
    /// Published protocol whose defaults create this draft. May be combined with hypothesis, tags, teams, related links, and date overrides. Omit subject_type_id, decision_metrics, variants, warehouse_exposure_configuration, datadog_flag_configuration, traffic_exposure, and structured_metadata.
    #[serde(rename = "protocol_id")]
    pub protocol_id: Option<String>,
    /// External links associated with the experiment.
    #[serde(rename = "related_links")]
    pub related_links: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>>,
    /// Complete Datadog split-by selection. Identify each property by column_name. Omit this field to copy organization defaults.
    #[serde(rename = "split_by_properties")]
    pub split_by_properties: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesSplitByPropertiesItems>>,
    /// Custom metadata fields and their values for the experiment.
    #[serde(rename = "structured_metadata")]
    pub structured_metadata: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems>>,
    /// Canonical ID of an existing subject type. Optional; defaults to the organization default.
    #[serde(rename = "subject_type_id")]
    pub subject_type_id: Option<uuid::Uuid>,
    /// Summary of the experiment. Optional and free-form.
    #[serde(rename = "summary")]
    pub summary: Option<String>,
    /// Non-team tag names to apply. Optional.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team handles that own the experiment. Optional.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    /// Traffic exposure fraction or schedule configured for the experiment.
    #[serde(rename = "traffic_exposure")]
    pub traffic_exposure: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure>,
    /// Variants selected for the experiment and their traffic allocations.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems>>,
    /// Warehouse model and experiment key used to read assignment data.
    #[serde(rename = "warehouse_exposure_configuration", default, with = "::serde_with::rust::double_option")]
    pub warehouse_exposure_configuration: Option<Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesWarehouseExposureConfiguration>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsCreateExperimentV2RequestDataAttributes {
    pub fn new(name: String) -> ExperimentsCreateExperimentV2RequestDataAttributes {
        ExperimentsCreateExperimentV2RequestDataAttributes {
            assignments_end_date: None,
            assignments_start_date: None,
            datadog_flag_configuration: None,
            decision_metrics: None,
            events_end_date: None,
            events_start_date: None,
            hypothesis: None,
            migration_metadata: None,
            name,
            protocol_id: None,
            related_links: None,
            split_by_properties: None,
            structured_metadata: None,
            subject_type_id: None,
            summary: None,
            tags: None,
            teams: None,
            traffic_exposure: None,
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

    pub fn datadog_flag_configuration(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration>,
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

    pub fn events_end_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.events_end_date = Some(value);
        self
    }

    pub fn events_start_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.events_start_date = Some(value);
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

    pub fn split_by_properties(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesSplitByPropertiesItems>,
    ) -> Self {
        self.split_by_properties = Some(value);
        self
    }

    pub fn structured_metadata(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems>,
    ) -> Self {
        self.structured_metadata = Some(value);
        self
    }

    pub fn subject_type_id(mut self, value: uuid::Uuid) -> Self {
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

    pub fn variants(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems>,
    ) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn warehouse_exposure_configuration(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesWarehouseExposureConfiguration>,
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

impl<'de> Deserialize<'de> for ExperimentsCreateExperimentV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsCreateExperimentV2RequestDataAttributesVisitor {
            type Value = ExperimentsCreateExperimentV2RequestDataAttributes;

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
                let mut datadog_flag_configuration: Option<Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration>> = None;
                let mut decision_metrics: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems>> = None;
                let mut events_end_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut events_start_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut hypothesis: Option<String> = None;
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut protocol_id: Option<String> = None;
                let mut related_links: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>> = None;
                let mut split_by_properties: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesSplitByPropertiesItems>> = None;
                let mut structured_metadata: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesStructuredMetadataItems>> = None;
                let mut subject_type_id: Option<uuid::Uuid> = None;
                let mut summary: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut traffic_exposure: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesTrafficExposure> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems>> = None;
                let mut warehouse_exposure_configuration: Option<Option<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesWarehouseExposureConfiguration>> = None;
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
                        "events_end_date" => {
                            events_end_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events_start_date" => {
                            events_start_date =
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
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "split_by_properties" => {
                            if v.is_null() {
                                continue;
                            }
                            split_by_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "structured_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            structured_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_type_id" => {
                            if v.is_null() {
                                continue;
                            }
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ExperimentsCreateExperimentV2RequestDataAttributes {
                    assignments_end_date,
                    assignments_start_date,
                    datadog_flag_configuration,
                    decision_metrics,
                    events_end_date,
                    events_start_date,
                    hypothesis,
                    migration_metadata,
                    name,
                    protocol_id,
                    related_links,
                    split_by_properties,
                    structured_metadata,
                    subject_type_id,
                    summary,
                    tags,
                    teams,
                    traffic_exposure,
                    variants,
                    warehouse_exposure_configuration,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsCreateExperimentV2RequestDataAttributesVisitor)
    }
}
