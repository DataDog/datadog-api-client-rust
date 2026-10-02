// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Diagnostic check results and their evaluation state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentDiagnosticsV2DTODataAttributes {
    /// Results of individual diagnostic checks.
    #[serde(rename = "diagnostics")]
    pub diagnostics: Vec<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems>,
    /// Time when the diagnostic checks were evaluated.
    #[serde(rename = "evaluated_at", default, with = "::serde_with::rust::double_option")]
    pub evaluated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Overall result of the experiment diagnostic checks.
    #[serde(rename = "result", default, with = "::serde_with::rust::double_option")]
    pub result: Option<Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesResult>>,
    /// Current state of the diagnostic evaluation.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesState,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsExperimentDiagnosticsV2DTODataAttributes {
    pub fn new(
        diagnostics: Vec<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems>,
        state: crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesState,
    ) -> ExperimentsExperimentDiagnosticsV2DTODataAttributes {
        ExperimentsExperimentDiagnosticsV2DTODataAttributes {
            diagnostics,
            evaluated_at: None,
            result: None,
            state,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn evaluated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.evaluated_at = Some(value);
        self
    }

    pub fn result(
        mut self,
        value: Option<
            crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesResult,
        >,
    ) -> Self {
        self.result = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsExperimentDiagnosticsV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentDiagnosticsV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentDiagnosticsV2DTODataAttributesVisitor {
            type Value = ExperimentsExperimentDiagnosticsV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut diagnostics: Option<Vec<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesDiagnosticsItems>> = None;
                let mut evaluated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut result: Option<Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesResult>> = None;
                let mut state: Option<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesState> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "diagnostics" => {
                            diagnostics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluated_at" => {
                            evaluated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result" => {
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _result) = result {
                                match _result {
                                    Some(crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesResult::UnparsedObject(_result)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTODataAttributesState::UnparsedObject(_state) => {
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
                let diagnostics =
                    diagnostics.ok_or_else(|| M::Error::missing_field("diagnostics"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = ExperimentsExperimentDiagnosticsV2DTODataAttributes {
                    diagnostics,
                    evaluated_at,
                    result,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsExperimentDiagnosticsV2DTODataAttributesVisitor)
    }
}
