// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an investigation in a list. To get an investigation's full content, use the get investigation
/// endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationListItemAttributes {
    /// When the investigation started.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether this is the latest investigation of the signal. Always true when `filter[signal_id]` isn't set.
    #[serde(rename = "is_latest")]
    pub is_latest: bool,
    /// ID of the detection rule that generated the signal. It can be empty while the investigation runs.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// Summary of the signal the investigation is about.
    #[serde(rename = "signal")]
    pub signal: Option<crate::datadogV2::model::SecurityAgentInvestigationSignal>,
    /// ID of the signal, in its V1 form.
    #[serde(rename = "signal_id")]
    pub signal_id: String,
    /// State of the investigation: `investigating` while it runs, `completed` after it has a result, or `failed`
    /// when it produced no result before its timeout.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::SecurityAgentInvestigationState,
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

impl SecurityAgentInvestigationListItemAttributes {
    pub fn new(
        is_latest: bool,
        rule_id: String,
        signal_id: String,
        state: crate::datadogV2::model::SecurityAgentInvestigationState,
    ) -> SecurityAgentInvestigationListItemAttributes {
        SecurityAgentInvestigationListItemAttributes {
            created_at: None,
            is_latest,
            rule_id,
            signal: None,
            signal_id,
            state,
            updated_at: None,
            verdict: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn signal(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationSignal,
    ) -> Self {
        self.signal = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationListItemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationListItemAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationListItemAttributesVisitor {
            type Value = SecurityAgentInvestigationListItemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut is_latest: Option<bool> = None;
                let mut rule_id: Option<String> = None;
                let mut signal: Option<crate::datadogV2::model::SecurityAgentInvestigationSignal> =
                    None;
                let mut signal_id: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::SecurityAgentInvestigationState> =
                    None;
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
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_latest" => {
                            is_latest = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal" => {
                            if v.is_null() {
                                continue;
                            }
                            signal = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_id" => {
                            signal_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
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
                let is_latest = is_latest.ok_or_else(|| M::Error::missing_field("is_latest"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let signal_id = signal_id.ok_or_else(|| M::Error::missing_field("signal_id"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = SecurityAgentInvestigationListItemAttributes {
                    created_at,
                    is_latest,
                    rule_id,
                    signal,
                    signal_id,
                    state,
                    updated_at,
                    verdict,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationListItemAttributesVisitor)
    }
}
