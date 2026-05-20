// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The matching experimentation entities grouped by type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationSearchResults {
    /// Matching dataset records. Present when `dataset_records` is included in `filter.scope`.
    #[serde(
        rename = "dataset_records",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dataset_records:
        Option<Option<Vec<crate::datadogV2::model::LLMObsDatasetRecordDataResponse>>>,
    /// Matching datasets. Present when `datasets` is included in `filter.scope`.
    #[serde(
        rename = "datasets",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub datasets: Option<Option<Vec<crate::datadogV2::model::LLMObsDatasetDataResponse>>>,
    /// Matching experiment runs. Present when `experiment_runs` is included in `filter.scope`.
    #[serde(
        rename = "experiment_runs",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_runs:
        Option<Option<Vec<crate::datadogV2::model::LLMObsExperimentRunDataResponse>>>,
    /// Matching experiments. Present when `experiments` is included in `filter.scope`.
    #[serde(
        rename = "experiments",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiments:
        Option<Option<Vec<crate::datadogV2::model::LLMObsExperimentDataAttributesResponse>>>,
    /// Matching projects. Present when `projects` is included in `filter.scope`.
    #[serde(
        rename = "projects",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub projects: Option<Option<Vec<crate::datadogV2::model::LLMObsProjectDataResponse>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationSearchResults {
    pub fn new() -> LLMObsExperimentationSearchResults {
        LLMObsExperimentationSearchResults {
            dataset_records: None,
            datasets: None,
            experiment_runs: None,
            experiments: None,
            projects: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dataset_records(
        mut self,
        value: Option<Vec<crate::datadogV2::model::LLMObsDatasetRecordDataResponse>>,
    ) -> Self {
        self.dataset_records = Some(value);
        self
    }

    pub fn datasets(
        mut self,
        value: Option<Vec<crate::datadogV2::model::LLMObsDatasetDataResponse>>,
    ) -> Self {
        self.datasets = Some(value);
        self
    }

    pub fn experiment_runs(
        mut self,
        value: Option<Vec<crate::datadogV2::model::LLMObsExperimentRunDataResponse>>,
    ) -> Self {
        self.experiment_runs = Some(value);
        self
    }

    pub fn experiments(
        mut self,
        value: Option<Vec<crate::datadogV2::model::LLMObsExperimentDataAttributesResponse>>,
    ) -> Self {
        self.experiments = Some(value);
        self
    }

    pub fn projects(
        mut self,
        value: Option<Vec<crate::datadogV2::model::LLMObsProjectDataResponse>>,
    ) -> Self {
        self.projects = Some(value);
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

impl Default for LLMObsExperimentationSearchResults {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsExperimentationSearchResults {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationSearchResultsVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationSearchResultsVisitor {
            type Value = LLMObsExperimentationSearchResults;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_records: Option<
                    Option<Vec<crate::datadogV2::model::LLMObsDatasetRecordDataResponse>>,
                > = None;
                let mut datasets: Option<
                    Option<Vec<crate::datadogV2::model::LLMObsDatasetDataResponse>>,
                > = None;
                let mut experiment_runs: Option<
                    Option<Vec<crate::datadogV2::model::LLMObsExperimentRunDataResponse>>,
                > = None;
                let mut experiments: Option<
                    Option<Vec<crate::datadogV2::model::LLMObsExperimentDataAttributesResponse>>,
                > = None;
                let mut projects: Option<
                    Option<Vec<crate::datadogV2::model::LLMObsProjectDataResponse>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataset_records" => {
                            dataset_records =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datasets" => {
                            datasets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_runs" => {
                            experiment_runs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiments" => {
                            experiments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projects" => {
                            projects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsExperimentationSearchResults {
                    dataset_records,
                    datasets,
                    experiment_runs,
                    experiments,
                    projects,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationSearchResultsVisitor)
    }
}
