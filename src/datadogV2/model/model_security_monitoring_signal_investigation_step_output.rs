// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Output from an investigation step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalInvestigationStepOutput {
    /// A one-line summary of the step analysis.
    #[serde(rename = "currentStepAnalysisOneliner")]
    pub current_step_analysis_oneliner: Option<String>,
    /// A detailed summary of the step analysis.
    #[serde(rename = "currentStepAnalysisSummary")]
    pub current_step_analysis_summary: String,
    /// The name of the investigation step.
    #[serde(rename = "name")]
    pub name: String,
    /// The verdict from the investigation step.
    #[serde(rename = "verdict")]
    pub verdict: crate::datadogV2::model::SecurityMonitoringSignalInvestigationStepOutputVerdict,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalInvestigationStepOutput {
    pub fn new(
        current_step_analysis_summary: String,
        name: String,
        verdict: crate::datadogV2::model::SecurityMonitoringSignalInvestigationStepOutputVerdict,
    ) -> SecurityMonitoringSignalInvestigationStepOutput {
        SecurityMonitoringSignalInvestigationStepOutput {
            current_step_analysis_oneliner: None,
            current_step_analysis_summary,
            name,
            verdict,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn current_step_analysis_oneliner(mut self, value: String) -> Self {
        self.current_step_analysis_oneliner = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalInvestigationStepOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalInvestigationStepOutputVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalInvestigationStepOutputVisitor {
            type Value = SecurityMonitoringSignalInvestigationStepOutput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_step_analysis_oneliner: Option<String> = None;
                let mut current_step_analysis_summary: Option<String> = None;
                let mut name: Option<String> = None;
                let mut verdict: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalInvestigationStepOutputVerdict,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "currentStepAnalysisOneliner" => {
                            if v.is_null() {
                                continue;
                            }
                            current_step_analysis_oneliner =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "currentStepAnalysisSummary" => {
                            current_step_analysis_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "verdict" => {
                            verdict = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _verdict) = verdict {
                                match _verdict {
                                    crate::datadogV2::model::SecurityMonitoringSignalInvestigationStepOutputVerdict::UnparsedObject(_verdict) => {
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
                let current_step_analysis_summary = current_step_analysis_summary
                    .ok_or_else(|| M::Error::missing_field("current_step_analysis_summary"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let verdict = verdict.ok_or_else(|| M::Error::missing_field("verdict"))?;

                let content = SecurityMonitoringSignalInvestigationStepOutput {
                    current_step_analysis_oneliner,
                    current_step_analysis_summary,
                    name,
                    verdict,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalInvestigationStepOutputVisitor)
    }
}
