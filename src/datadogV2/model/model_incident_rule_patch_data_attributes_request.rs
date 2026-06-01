// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for patching an incident rule. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRulePatchDataAttributesRequest {
    /// A query-based condition for an incident rule.
    #[serde(rename = "condition")]
    pub condition: Option<crate::datadogV2::model::IncidentRuleQueryCondition>,
    /// List of field-based conditions.
    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>>,
    /// Whether the rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The JSON-encoded payload for the task.
    #[serde(rename = "task_payload")]
    pub task_payload: Option<String>,
    /// The trigger event for an incident rule.
    #[serde(rename = "trigger")]
    pub trigger: Option<crate::datadogV2::model::IncidentRuleTriggerType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRulePatchDataAttributesRequest {
    pub fn new() -> IncidentRulePatchDataAttributesRequest {
        IncidentRulePatchDataAttributesRequest {
            condition: None,
            conditions: None,
            enabled: None,
            task_payload: None,
            trigger: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn condition(mut self, value: crate::datadogV2::model::IncidentRuleQueryCondition) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn conditions(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentRuleCondition>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn task_payload(mut self, value: String) -> Self {
        self.task_payload = Some(value);
        self
    }

    pub fn trigger(mut self, value: crate::datadogV2::model::IncidentRuleTriggerType) -> Self {
        self.trigger = Some(value);
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

impl Default for IncidentRulePatchDataAttributesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentRulePatchDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRulePatchDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentRulePatchDataAttributesRequestVisitor {
            type Value = IncidentRulePatchDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<crate::datadogV2::model::IncidentRuleQueryCondition> =
                    None;
                let mut conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>> =
                    None;
                let mut enabled: Option<bool> = None;
                let mut task_payload: Option<String> = None;
                let mut trigger: Option<crate::datadogV2::model::IncidentRuleTriggerType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "condition" => {
                            if v.is_null() {
                                continue;
                            }
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditions" => {
                            if v.is_null() {
                                continue;
                            }
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_payload" => {
                            if v.is_null() {
                                continue;
                            }
                            task_payload =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger" => {
                            if v.is_null() {
                                continue;
                            }
                            trigger = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trigger) = trigger {
                                match _trigger {
                                    crate::datadogV2::model::IncidentRuleTriggerType::UnparsedObject(_trigger) => {
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

                let content = IncidentRulePatchDataAttributesRequest {
                    condition,
                    conditions,
                    enabled,
                    task_payload,
                    trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRulePatchDataAttributesRequestVisitor)
    }
}
