// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Schedule that controls traffic exposure for experiments created from the protocol.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
    /// Whether the exposure schedule starts automatically.
    #[serde(rename = "autostart")]
    pub autostart: Option<bool>,
    /// Action taken when a guardrail triggers during the exposure schedule.
    #[serde(rename = "guardrail_triggered_action")]
    pub guardrail_triggered_action: Option<String>,
    /// Ordered steps that define changes in traffic exposure.
    #[serde(rename = "rollout_steps")]
    pub rollout_steps: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems>>,
    /// Interval between traffic selections, in milliseconds.
    #[serde(rename = "selection_interval_ms")]
    pub selection_interval_ms: Option<i64>,
    /// Method used to increase traffic exposure over the schedule.
    #[serde(rename = "strategy")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
    pub fn new() -> ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
        ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
            autostart: None,
            guardrail_triggered_action: None,
            rollout_steps: None,
            selection_interval_ms: None,
            strategy: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn autostart(mut self, value: bool) -> Self {
        self.autostart = Some(value);
        self
    }

    pub fn guardrail_triggered_action(mut self, value: String) -> Self {
        self.guardrail_triggered_action = Some(value);
        self
    }

    pub fn rollout_steps(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems>,
    ) -> Self {
        self.rollout_steps = Some(value);
        self
    }

    pub fn selection_interval_ms(mut self, value: i64) -> Self {
        self.selection_interval_ms = Some(value);
        self
    }

    pub fn strategy(mut self, value: String) -> Self {
        self.strategy = Some(value);
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

impl Default for ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPublicProtocolResponseDataAttributesExposureScheduleVisitor;
        impl<'a> Visitor<'a> for ExperimentsPublicProtocolResponseDataAttributesExposureScheduleVisitor {
            type Value = ExperimentsPublicProtocolResponseDataAttributesExposureSchedule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut autostart: Option<bool> = None;
                let mut guardrail_triggered_action: Option<String> = None;
                let mut rollout_steps: Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesExposureScheduleRolloutStepsItems>> = None;
                let mut selection_interval_ms: Option<i64> = None;
                let mut strategy: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "autostart" => {
                            if v.is_null() {
                                continue;
                            }
                            autostart = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_triggered_action" => {
                            if v.is_null() {
                                continue;
                            }
                            guardrail_triggered_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rollout_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            rollout_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selection_interval_ms" => {
                            if v.is_null() {
                                continue;
                            }
                            selection_interval_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            if v.is_null() {
                                continue;
                            }
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPublicProtocolResponseDataAttributesExposureSchedule {
                    autostart,
                    guardrail_triggered_action,
                    rollout_steps,
                    selection_interval_ms,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsPublicProtocolResponseDataAttributesExposureScheduleVisitor)
    }
}
