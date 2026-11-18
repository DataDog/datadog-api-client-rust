// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Selected experiment result run, its recorded EVSI forecast, and results freshness.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentResultsV2MetaDTO {
    /// Forecast horizon and traffic counts used to calculate expected value of sample information (EVSI) for result_run_id. Assumes traffic continues at the rate observed during the lookback window. Omitted if the forecast was not recorded or could not be loaded.
    #[serde(rename = "evsi_context")]
    pub evsi_context: Option<crate::datadogV2::model::ExperimentsEVSIContext>,
    /// Whether the saved results require a refresh or their freshness cannot be confirmed. See stale_reasons for
    /// details.
    #[serde(rename = "is_stale")]
    pub is_stale: Option<bool>,
    /// ID of the successful experiment run that produced these results.
    #[serde(rename = "result_run_id")]
    pub result_run_id: uuid::Uuid,
    /// Time when the experiment results were last updated.
    #[serde(
        rename = "results_last_updated",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Reasons the saved experiment results are stale.
    #[serde(rename = "stale_reasons")]
    pub stale_reasons: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExperimentResultsV2MetaDTO {
    pub fn new(result_run_id: uuid::Uuid) -> ExperimentsExperimentResultsV2MetaDTO {
        ExperimentsExperimentResultsV2MetaDTO {
            evsi_context: None,
            is_stale: None,
            result_run_id,
            results_last_updated: None,
            stale_reasons: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn evsi_context(mut self, value: crate::datadogV2::model::ExperimentsEVSIContext) -> Self {
        self.evsi_context = Some(value);
        self
    }

    pub fn is_stale(mut self, value: bool) -> Self {
        self.is_stale = Some(value);
        self
    }

    pub fn results_last_updated(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.results_last_updated = Some(value);
        self
    }

    pub fn stale_reasons(mut self, value: Vec<String>) -> Self {
        self.stale_reasons = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsExperimentResultsV2MetaDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentResultsV2MetaDTOVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentResultsV2MetaDTOVisitor {
            type Value = ExperimentsExperimentResultsV2MetaDTO;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut evsi_context: Option<crate::datadogV2::model::ExperimentsEVSIContext> =
                    None;
                let mut is_stale: Option<bool> = None;
                let mut result_run_id: Option<uuid::Uuid> = None;
                let mut results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut stale_reasons: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "evsi_context" => {
                            if v.is_null() {
                                continue;
                            }
                            evsi_context =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_stale" => {
                            if v.is_null() {
                                continue;
                            }
                            is_stale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_run_id" => {
                            result_run_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results_last_updated" => {
                            results_last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stale_reasons" => {
                            if v.is_null() {
                                continue;
                            }
                            stale_reasons =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let result_run_id =
                    result_run_id.ok_or_else(|| M::Error::missing_field("result_run_id"))?;

                let content = ExperimentsExperimentResultsV2MetaDTO {
                    evsi_context,
                    is_stale,
                    result_run_id,
                    results_last_updated,
                    stale_reasons,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExperimentResultsV2MetaDTOVisitor)
    }
}
