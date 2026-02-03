// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a signal investigation response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalInvestigationResponseAttributes {
    /// The unique ID of the investigation.
    #[serde(rename = "investigation_id")]
    pub investigation_id: String,
    /// The ID of the rule that triggered the signal.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// The unique ID of the security signal.
    #[serde(rename = "signal_id")]
    pub signal_id: String,
    /// The state of the investigation.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::SecurityMonitoringSignalInvestigationState>,
    /// Information about an investigation step.
    #[serde(rename = "step")]
    pub step: Option<crate::datadogV2::model::SecurityMonitoringSignalInvestigationStep>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalInvestigationResponseAttributes {
    pub fn new(
        investigation_id: String,
        rule_id: String,
        signal_id: String,
    ) -> SecurityMonitoringSignalInvestigationResponseAttributes {
        SecurityMonitoringSignalInvestigationResponseAttributes {
            investigation_id,
            rule_id,
            signal_id,
            state: None,
            step: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn state(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalInvestigationState,
    ) -> Self {
        self.state = Some(value);
        self
    }

    pub fn step(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalInvestigationStep,
    ) -> Self {
        self.step = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalInvestigationResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalInvestigationResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalInvestigationResponseAttributesVisitor {
            type Value = SecurityMonitoringSignalInvestigationResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut investigation_id: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut signal_id: Option<String> = None;
                let mut state: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalInvestigationState,
                > = None;
                let mut step: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalInvestigationStep,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "investigation_id" => {
                            investigation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_id" => {
                            signal_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::SecurityMonitoringSignalInvestigationState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "step" => {
                            if v.is_null() {
                                continue;
                            }
                            step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let investigation_id =
                    investigation_id.ok_or_else(|| M::Error::missing_field("investigation_id"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let signal_id = signal_id.ok_or_else(|| M::Error::missing_field("signal_id"))?;

                let content = SecurityMonitoringSignalInvestigationResponseAttributes {
                    investigation_id,
                    rule_id,
                    signal_id,
                    state,
                    step,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalInvestigationResponseAttributesVisitor)
    }
}
