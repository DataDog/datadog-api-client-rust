// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Refresh requirements and warnings returned by an experiment update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2MetaDTO {
    /// Whether this edit needs a full or non-full pipeline run. This operation does not start the run. A later false value does not clear a refresh required by an earlier edit.
    #[serde(rename = "needs_pipeline_refresh")]
    pub needs_pipeline_refresh: bool,
    /// POST to this endpoint after finishing your edits. The full_refresh query parameter selects the required run type. Across multiple edits any full_refresh=true requirement takes priority.
    #[serde(rename = "refresh_endpoint")]
    pub refresh_endpoint: Option<String>,
    /// Warnings returned after the experiment update.
    #[serde(rename = "warnings")]
    pub warnings:
        Option<Vec<crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTOWarningsItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchExperimentV2MetaDTO {
    pub fn new(needs_pipeline_refresh: bool) -> ExperimentsPatchExperimentV2MetaDTO {
        ExperimentsPatchExperimentV2MetaDTO {
            needs_pipeline_refresh,
            refresh_endpoint: None,
            warnings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn refresh_endpoint(mut self, value: String) -> Self {
        self.refresh_endpoint = Some(value);
        self
    }

    pub fn warnings(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTOWarningsItems>,
    ) -> Self {
        self.warnings = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2MetaDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2MetaDTOVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2MetaDTOVisitor {
            type Value = ExperimentsPatchExperimentV2MetaDTO;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut needs_pipeline_refresh: Option<bool> = None;
                let mut refresh_endpoint: Option<String> = None;
                let mut warnings: Option<
                    Vec<crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTOWarningsItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "needs_pipeline_refresh" => {
                            needs_pipeline_refresh =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "refresh_endpoint" => {
                            if v.is_null() {
                                continue;
                            }
                            refresh_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let needs_pipeline_refresh = needs_pipeline_refresh
                    .ok_or_else(|| M::Error::missing_field("needs_pipeline_refresh"))?;

                let content = ExperimentsPatchExperimentV2MetaDTO {
                    needs_pipeline_refresh,
                    refresh_endpoint,
                    warnings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchExperimentV2MetaDTOVisitor)
    }
}
