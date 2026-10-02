// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary fields for an experiment returned in a list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentV2ListDTODataAttributes {
    /// End of the window used to read experiment assignments.
    #[serde(rename = "assignments_end_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the window used to read experiment assignments.
    #[serde(rename = "assignments_start_date", default, with = "::serde_with::rust::double_option")]
    pub assignments_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Time when the experiment was concluded.
    #[serde(rename = "concluded_at", default, with = "::serde_with::rust::double_option")]
    pub concluded_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Outcome and supporting text recorded when the experiment is concluded.
    #[serde(rename = "conclusion")]
    pub conclusion: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesConclusion>,
    /// Time when the experiment was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// End of the window used to read metric events.
    #[serde(rename = "events_end_date", default, with = "::serde_with::rust::double_option")]
    pub events_end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Start of the window used to read metric events.
    #[serde(rename = "events_start_date", default, with = "::serde_with::rust::double_option")]
    pub events_start_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Kind of experiment. STANDARD is an ordinary experiment. Other values, such as CANARY and HOLDOUT, identify experiments owned by another workflow. New kinds may be added; treat unknown values as non-standard.
    #[serde(rename = "experiment_type")]
    pub experiment_type: Option<String>,
    /// Expected effect that the experiment is designed to test.
    #[serde(rename = "hypothesis")]
    pub hypothesis: Option<String>,
    /// Metadata associated with migration of this resource.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the experiment.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Suffix used for the experiment tables in the analysis pipeline.
    #[serde(rename = "pipeline_table_suffix")]
    pub pipeline_table_suffix: Option<String>,
    /// Identifier of the protocol associated with the experiment.
    #[serde(rename = "protocol_id")]
    pub protocol_id: Option<String>,
    /// External links associated with the experiment.
    #[serde(rename = "related_links")]
    pub related_links: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesRelatedLinksItems>>,
    /// Time when the experiment results were last updated.
    #[serde(rename = "results_last_updated", default, with = "::serde_with::rust::double_option")]
    pub results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Current stage in the experiment lifecycle.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesStatus>,
    /// Custom metadata fields and their values for the experiment.
    #[serde(rename = "structured_metadata")]
    pub structured_metadata: Option<Vec<crate::datadogV2::model::ExperimentsStructuredMetadataResponse>>,
    /// Identifier of the subject type used for experiment assignments.
    #[serde(rename = "subject_type_id", default, with = "::serde_with::rust::double_option")]
    pub subject_type_id: Option<Option<String>>,
    /// Free-form summary of the experiment.
    #[serde(rename = "summary")]
    pub summary: Option<String>,
    /// Tag names associated with the experiment.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Team handles associated with the experiment.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    /// Time when the experiment was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsExperimentV2ListDTODataAttributes {
    pub fn new() -> ExperimentsExperimentV2ListDTODataAttributes {
        ExperimentsExperimentV2ListDTODataAttributes {
            assignments_end_date: None,
            assignments_start_date: None,
            concluded_at: None,
            conclusion: None,
            created_at: None,
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
            status: None,
            structured_metadata: None,
            subject_type_id: None,
            summary: None,
            tags: None,
            teams: None,
            updated_at: None,
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

impl Default for ExperimentsExperimentV2ListDTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsExperimentV2ListDTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentV2ListDTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentV2ListDTODataAttributesVisitor {
            type Value = ExperimentsExperimentV2ListDTODataAttributes;

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
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
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

                let content = ExperimentsExperimentV2ListDTODataAttributes {
                    assignments_end_date,
                    assignments_start_date,
                    concluded_at,
                    conclusion,
                    created_at,
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
                    status,
                    structured_metadata,
                    subject_type_id,
                    summary,
                    tags,
                    teams,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExperimentV2ListDTODataAttributesVisitor)
    }
}
