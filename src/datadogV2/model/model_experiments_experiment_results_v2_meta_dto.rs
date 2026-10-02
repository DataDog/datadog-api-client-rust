// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about when experiment results were updated and whether they are stale.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentResultsV2MetaDTO {
    /// Whether the saved results require a refresh or their freshness cannot be confirmed. See stale_reasons for
    /// details.
    #[serde(rename = "is_stale")]
    pub is_stale: Option<bool>,
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
    pub fn new() -> ExperimentsExperimentResultsV2MetaDTO {
        ExperimentsExperimentResultsV2MetaDTO {
            is_stale: None,
            results_last_updated: None,
            stale_reasons: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl Default for ExperimentsExperimentResultsV2MetaDTO {
    fn default() -> Self {
        Self::new()
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
                let mut is_stale: Option<bool> = None;
                let mut results_last_updated: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut stale_reasons: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_stale" => {
                            if v.is_null() {
                                continue;
                            }
                            is_stale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ExperimentsExperimentResultsV2MetaDTO {
                    is_stale,
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
