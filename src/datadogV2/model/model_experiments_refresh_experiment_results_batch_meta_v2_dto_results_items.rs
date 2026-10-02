// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Refresh outcome for one experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
    /// ID of the experiment associated with this result.
    #[serde(rename = "experiment_id")]
    pub experiment_id: Option<String>,
    /// Outcome of attempting to refresh one experiment.
    #[serde(rename = "outcome")]
    pub outcome: Option<crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
    pub fn new() -> ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
        ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
            experiment_id: None,
            outcome: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn experiment_id(mut self, value: String) -> Self {
        self.experiment_id = Some(value);
        self
    }

    pub fn outcome(
        mut self,
        value: crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome,
    ) -> Self {
        self.outcome = Some(value);
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

impl Default for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsVisitor {
            type Value = ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut experiment_id: Option<String> = None;
                let mut outcome: Option<crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "experiment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outcome" => {
                            if v.is_null() {
                                continue;
                            }
                            outcome = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _outcome) = outcome {
                                match _outcome {
                                    crate::datadogV2::model::ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsOutcome::UnparsedObject(_outcome) => {
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

                let content = ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItems {
                    experiment_id,
                    outcome,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsRefreshExperimentResultsBatchMetaV2DTOResultsItemsVisitor)
    }
}
