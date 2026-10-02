// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Outcome and supporting text recorded when the experiment is concluded.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
    /// Reason for the recorded decision.
    #[serde(
        rename = "decision_reason",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub decision_reason: Option<Option<String>>,
    /// Recorded experiment outcome.
    #[serde(
        rename = "outcome",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub outcome: Option<
        Option<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesConclusionOutcome>,
    >,
    /// Summary of the experiment conclusion.
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub summary: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
    pub fn new() -> ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
        ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
            decision_reason: None,
            outcome: None,
            summary: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn decision_reason(mut self, value: Option<String>) -> Self {
        self.decision_reason = Some(value);
        self
    }

    pub fn outcome(
        mut self,
        value: Option<
            crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesConclusionOutcome,
        >,
    ) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn summary(mut self, value: Option<String>) -> Self {
        self.summary = Some(value);
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

impl Default for ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesConclusionVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2ResponseDataAttributesConclusionVisitor {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributesConclusion;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut decision_reason: Option<Option<String>> = None;
                let mut outcome: Option<Option<crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesConclusionOutcome>> = None;
                let mut summary: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "decision_reason" => {
                            decision_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outcome" => {
                            outcome = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _outcome) = outcome {
                                match _outcome {
                                    Some(crate::datadogV2::model::ExperimentsExperimentV2DTODataAttributesConclusionOutcome::UnparsedObject(_outcome)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPatchExperimentV2ResponseDataAttributesConclusion {
                    decision_reason,
                    outcome,
                    summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsPatchExperimentV2ResponseDataAttributesConclusionVisitor)
    }
}
