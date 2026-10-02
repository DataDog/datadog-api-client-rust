// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of refresh outcomes across the organization's experiments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
    /// Number of experiments updated by the refresh request.
    #[serde(rename = "experiments_updated")]
    pub experiments_updated: Option<i64>,
    /// Refresh outcome reported for each experiment.
    #[serde(rename = "results")]
    pub results: Option<Vec<Option<crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
    pub fn new() -> ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
        ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
            experiments_updated: None,
            results: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn experiments_updated(mut self, value: i64) -> Self {
        self.experiments_updated = Some(value);
        self
    }

    pub fn results(
        mut self,
        value: Vec<Option<crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems>>,
    ) -> Self {
        self.results = Some(value);
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

impl Default for ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsRefreshExperimentResultsBatchMetaV2DTOVisitor;
        impl<'a> Visitor<'a> for ExperimentsRefreshExperimentResultsBatchMetaV2DTOVisitor {
            type Value = ExperimentsRefreshExperimentResultsBatchMetaV2DTO;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut experiments_updated: Option<i64> = None;
                let mut results: Option<Vec<Option<crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "experiments_updated" => {
                            if v.is_null() {
                                continue;
                            }
                            experiments_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results" => {
                            if v.is_null() {
                                continue;
                            }
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsRefreshExperimentResultsBatchMetaV2DTO {
                    experiments_updated,
                    results,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsRefreshExperimentResultsBatchMetaV2DTOVisitor)
    }
}
