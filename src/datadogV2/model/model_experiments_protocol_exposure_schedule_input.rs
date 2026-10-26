// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Traffic schedule. Publishing and replacement require `NO_ROLLOUT` with exactly one step, or `UNIFORM_INTERVALS` with a positive selection interval and at least one step. Requires `environment_id`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsProtocolExposureScheduleInput {
    /// Whether the exposure schedule starts automatically.
    #[serde(rename = "autostart")]
    pub autostart: Option<bool>,
    /// Action taken when a guardrail triggers during the exposure schedule.
    #[serde(
        rename = "guardrail_triggered_action",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub guardrail_triggered_action: Option<Option<String>>,
    /// Rollout steps, replaced when supplied.
    #[serde(
        rename = "rollout_steps",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rollout_steps:
        Option<Option<Vec<crate::datadogV2::model::ExperimentsProtocolExposureStepInput>>>,
    /// Interval between traffic selections, in milliseconds.
    #[serde(
        rename = "selection_interval_ms",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub selection_interval_ms: Option<Option<i64>>,
    /// Method used to increase traffic exposure over the schedule.
    #[serde(rename = "strategy")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsProtocolExposureScheduleInput {
    pub fn new() -> ExperimentsProtocolExposureScheduleInput {
        ExperimentsProtocolExposureScheduleInput {
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

    pub fn guardrail_triggered_action(mut self, value: Option<String>) -> Self {
        self.guardrail_triggered_action = Some(value);
        self
    }

    pub fn rollout_steps(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ExperimentsProtocolExposureStepInput>>,
    ) -> Self {
        self.rollout_steps = Some(value);
        self
    }

    pub fn selection_interval_ms(mut self, value: Option<i64>) -> Self {
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

impl Default for ExperimentsProtocolExposureScheduleInput {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsProtocolExposureScheduleInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsProtocolExposureScheduleInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsProtocolExposureScheduleInputVisitor {
            type Value = ExperimentsProtocolExposureScheduleInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut autostart: Option<bool> = None;
                let mut guardrail_triggered_action: Option<Option<String>> = None;
                let mut rollout_steps: Option<
                    Option<Vec<crate::datadogV2::model::ExperimentsProtocolExposureStepInput>>,
                > = None;
                let mut selection_interval_ms: Option<Option<i64>> = None;
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
                            guardrail_triggered_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rollout_steps" => {
                            rollout_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "selection_interval_ms" => {
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

                let content = ExperimentsProtocolExposureScheduleInput {
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

        deserializer.deserialize_any(ExperimentsProtocolExposureScheduleInputVisitor)
    }
}
