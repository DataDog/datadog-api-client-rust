// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an investigation. When `fields[investigations]` is set, only listed attributes are returned.
/// Attributes the investigation doesn't have yet are absent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationAttributes {
    /// Summary of the investigation's findings.
    #[serde(rename = "conclusion")]
    pub conclusion: Option<String>,
    /// When the investigation started.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The entities the investigation found, by role.
    #[serde(rename = "key_entities")]
    pub key_entities: Option<crate::datadogV2::model::SecurityAgentInvestigationKeyEntities>,
    /// The evidence the verdict rests on.
    #[serde(rename = "key_evidence")]
    pub key_evidence: Option<Vec<crate::datadogV2::model::SecurityAgentInvestigationKeyEvidence>>,
    /// Suggested responses to the signal.
    #[serde(rename = "response_suggestions")]
    pub response_suggestions:
        Option<crate::datadogV2::model::SecurityAgentInvestigationResponseSuggestions>,
    /// ID of the detection rule that generated the signal.
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
    /// Summary of the signal the investigation is about.
    #[serde(rename = "signal")]
    pub signal: Option<crate::datadogV2::model::SecurityAgentInvestigationSignal>,
    /// ID of the signal, in its V1 form.
    #[serde(rename = "signal_id")]
    pub signal_id: Option<String>,
    /// State of the investigation: `investigating` while it runs, `completed` after it has a result, or `failed`
    /// when it produced no result before its timeout.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::SecurityAgentInvestigationState>,
    /// The steps of the investigation, in order.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::SecurityAgentInvestigationStep>>,
    /// When the investigation last changed.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Verdict of the investigation. Only completed investigations have one.
    #[serde(rename = "verdict")]
    pub verdict: Option<crate::datadogV2::model::SecurityAgentInvestigationVerdict>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationAttributes {
    pub fn new() -> SecurityAgentInvestigationAttributes {
        SecurityAgentInvestigationAttributes {
            conclusion: None,
            created_at: None,
            key_entities: None,
            key_evidence: None,
            response_suggestions: None,
            rule_id: None,
            signal: None,
            signal_id: None,
            state: None,
            steps: None,
            updated_at: None,
            verdict: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conclusion(mut self, value: String) -> Self {
        self.conclusion = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn key_entities(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationKeyEntities,
    ) -> Self {
        self.key_entities = Some(value);
        self
    }

    pub fn key_evidence(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityAgentInvestigationKeyEvidence>,
    ) -> Self {
        self.key_evidence = Some(value);
        self
    }

    pub fn response_suggestions(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationResponseSuggestions,
    ) -> Self {
        self.response_suggestions = Some(value);
        self
    }

    pub fn rule_id(mut self, value: String) -> Self {
        self.rule_id = Some(value);
        self
    }

    pub fn signal(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationSignal,
    ) -> Self {
        self.signal = Some(value);
        self
    }

    pub fn signal_id(mut self, value: String) -> Self {
        self.signal_id = Some(value);
        self
    }

    pub fn state(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationState,
    ) -> Self {
        self.state = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityAgentInvestigationStep>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn verdict(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationVerdict,
    ) -> Self {
        self.verdict = Some(value);
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

impl Default for SecurityAgentInvestigationAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityAgentInvestigationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationAttributesVisitor {
            type Value = SecurityAgentInvestigationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conclusion: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut key_entities: Option<
                    crate::datadogV2::model::SecurityAgentInvestigationKeyEntities,
                > = None;
                let mut key_evidence: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationKeyEvidence>,
                > = None;
                let mut response_suggestions: Option<
                    crate::datadogV2::model::SecurityAgentInvestigationResponseSuggestions,
                > = None;
                let mut rule_id: Option<String> = None;
                let mut signal: Option<crate::datadogV2::model::SecurityAgentInvestigationSignal> =
                    None;
                let mut signal_id: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::SecurityAgentInvestigationState> =
                    None;
                let mut steps: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationStep>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut verdict: Option<
                    crate::datadogV2::model::SecurityAgentInvestigationVerdict,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "key_entities" => {
                            if v.is_null() {
                                continue;
                            }
                            key_entities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_evidence" => {
                            if v.is_null() {
                                continue;
                            }
                            key_evidence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_suggestions" => {
                            if v.is_null() {
                                continue;
                            }
                            response_suggestions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal" => {
                            if v.is_null() {
                                continue;
                            }
                            signal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_id" => {
                            if v.is_null() {
                                continue;
                            }
                            signal_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::SecurityAgentInvestigationState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "verdict" => {
                            if v.is_null() {
                                continue;
                            }
                            verdict = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _verdict) = verdict {
                                match _verdict {
                                    crate::datadogV2::model::SecurityAgentInvestigationVerdict::UnparsedObject(_verdict) => {
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

                let content = SecurityAgentInvestigationAttributes {
                    conclusion,
                    created_at,
                    key_entities,
                    key_evidence,
                    response_suggestions,
                    rule_id,
                    signal,
                    signal_id,
                    state,
                    steps,
                    updated_at,
                    verdict,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationAttributesVisitor)
    }
}
