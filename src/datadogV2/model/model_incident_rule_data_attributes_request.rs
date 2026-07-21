// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an incident rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRuleDataAttributesRequest {
    /// A query-based condition for an incident rule.
    #[serde(rename = "condition")]
    pub condition: crate::datadogV2::model::IncidentRuleQueryCondition,
    /// The condition table type. 1 = raw query.
    #[serde(rename = "condition_table_type")]
    pub condition_table_type: i64,
    /// List of field-based conditions.
    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>>,
    /// Whether the rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The execution type of an incident rule.
    #[serde(rename = "execution_type")]
    pub execution_type: crate::datadogV2::model::IncidentRuleExecutionType,
    /// The UUID of the incident type this rule applies to.
    #[serde(
        rename = "incident_type_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub incident_type_uuid: Option<Option<uuid::Uuid>>,
    /// Whether any condition (OR logic) should match instead of all (AND logic).
    #[serde(rename = "match_any_condition")]
    pub match_any_condition: Option<bool>,
    /// The task ID for an incident rule.
    #[serde(rename = "task_id")]
    pub task_id: crate::datadogV2::model::IncidentRuleTaskIDType,
    /// The JSON-encoded payload for the task.
    #[serde(rename = "task_payload")]
    pub task_payload: String,
    /// The trigger event for an incident rule.
    #[serde(rename = "trigger")]
    pub trigger: Option<crate::datadogV2::model::IncidentRuleTriggerType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRuleDataAttributesRequest {
    pub fn new(
        condition: crate::datadogV2::model::IncidentRuleQueryCondition,
        condition_table_type: i64,
        enabled: bool,
        execution_type: crate::datadogV2::model::IncidentRuleExecutionType,
        task_id: crate::datadogV2::model::IncidentRuleTaskIDType,
        task_payload: String,
    ) -> IncidentRuleDataAttributesRequest {
        IncidentRuleDataAttributesRequest {
            condition,
            condition_table_type,
            conditions: None,
            enabled,
            execution_type,
            incident_type_uuid: None,
            match_any_condition: None,
            task_id,
            task_payload,
            trigger: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conditions(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentRuleCondition>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn incident_type_uuid(mut self, value: Option<uuid::Uuid>) -> Self {
        self.incident_type_uuid = Some(value);
        self
    }

    pub fn match_any_condition(mut self, value: bool) -> Self {
        self.match_any_condition = Some(value);
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

impl<'de> Deserialize<'de> for IncidentRuleDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRuleDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentRuleDataAttributesRequestVisitor {
            type Value = IncidentRuleDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<crate::datadogV2::model::IncidentRuleQueryCondition> =
                    None;
                let mut condition_table_type: Option<i64> = None;
                let mut conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>> =
                    None;
                let mut enabled: Option<bool> = None;
                let mut execution_type: Option<crate::datadogV2::model::IncidentRuleExecutionType> =
                    None;
                let mut incident_type_uuid: Option<Option<uuid::Uuid>> = None;
                let mut match_any_condition: Option<bool> = None;
                let mut task_id: Option<crate::datadogV2::model::IncidentRuleTaskIDType> = None;
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
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "condition_table_type" => {
                            condition_table_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditions" => {
                            if v.is_null() {
                                continue;
                            }
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_type" => {
                            execution_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _execution_type) = execution_type {
                                match _execution_type {
                                    crate::datadogV2::model::IncidentRuleExecutionType::UnparsedObject(_execution_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "incident_type_uuid" => {
                            incident_type_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_any_condition" => {
                            if v.is_null() {
                                continue;
                            }
                            match_any_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_id" => {
                            task_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _task_id) = task_id {
                                match _task_id {
                                    crate::datadogV2::model::IncidentRuleTaskIDType::UnparsedObject(_task_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "task_payload" => {
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
                let condition = condition.ok_or_else(|| M::Error::missing_field("condition"))?;
                let condition_table_type = condition_table_type
                    .ok_or_else(|| M::Error::missing_field("condition_table_type"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let execution_type =
                    execution_type.ok_or_else(|| M::Error::missing_field("execution_type"))?;
                let task_id = task_id.ok_or_else(|| M::Error::missing_field("task_id"))?;
                let task_payload =
                    task_payload.ok_or_else(|| M::Error::missing_field("task_payload"))?;

                let content = IncidentRuleDataAttributesRequest {
                    condition,
                    condition_table_type,
                    conditions,
                    enabled,
                    execution_type,
                    incident_type_uuid,
                    match_any_condition,
                    task_id,
                    task_payload,
                    trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRuleDataAttributesRequestVisitor)
    }
}
