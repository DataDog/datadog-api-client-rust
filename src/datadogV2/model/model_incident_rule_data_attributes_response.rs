// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident rule in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRuleDataAttributesResponse {
    /// A query-based condition for an incident rule.
    #[serde(rename = "condition")]
    pub condition: Option<crate::datadogV2::model::IncidentRuleQueryCondition>,
    /// The condition table type.
    #[serde(rename = "condition_table_type")]
    pub condition_table_type: Option<i32>,
    /// List of field-based conditions.
    #[serde(rename = "conditions")]
    pub conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>>,
    /// Timestamp when the rule was created.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who created the rule.
    #[serde(rename = "created_by_uuid")]
    pub created_by_uuid: Option<uuid::Uuid>,
    /// Timestamp when the rule was deleted.
    #[serde(
        rename = "deleted",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Whether the rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The execution type of the rule.
    #[serde(rename = "execution_type")]
    pub execution_type: Option<i32>,
    /// The incident settings association UUID.
    #[serde(
        rename = "incident_settings_association_uuid",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub incident_settings_association_uuid: Option<Option<uuid::Uuid>>,
    /// Whether any condition should match.
    #[serde(rename = "match_any_condition")]
    pub match_any_condition: Option<bool>,
    /// Timestamp when the rule was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who last modified the rule.
    #[serde(rename = "modified_by_uuid")]
    pub modified_by_uuid: Option<uuid::Uuid>,
    /// The organization ID.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// The task ID.
    #[serde(
        rename = "task_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub task_id: Option<Option<String>>,
    /// The JSON-encoded task payload.
    #[serde(
        rename = "task_payload",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub task_payload: Option<Option<String>>,
    /// The trigger event for the rule.
    #[serde(rename = "trigger")]
    pub trigger: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRuleDataAttributesResponse {
    pub fn new() -> IncidentRuleDataAttributesResponse {
        IncidentRuleDataAttributesResponse {
            condition: None,
            condition_table_type: None,
            conditions: None,
            created: None,
            created_by_uuid: None,
            deleted: None,
            enabled: None,
            execution_type: None,
            incident_settings_association_uuid: None,
            match_any_condition: None,
            modified: None,
            modified_by_uuid: None,
            org_id: None,
            task_id: None,
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

    pub fn condition_table_type(mut self, value: i32) -> Self {
        self.condition_table_type = Some(value);
        self
    }

    pub fn conditions(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentRuleCondition>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn created_by_uuid(mut self, value: uuid::Uuid) -> Self {
        self.created_by_uuid = Some(value);
        self
    }

    pub fn deleted(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn execution_type(mut self, value: i32) -> Self {
        self.execution_type = Some(value);
        self
    }

    pub fn incident_settings_association_uuid(mut self, value: Option<uuid::Uuid>) -> Self {
        self.incident_settings_association_uuid = Some(value);
        self
    }

    pub fn match_any_condition(mut self, value: bool) -> Self {
        self.match_any_condition = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn modified_by_uuid(mut self, value: uuid::Uuid) -> Self {
        self.modified_by_uuid = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn task_id(mut self, value: Option<String>) -> Self {
        self.task_id = Some(value);
        self
    }

    pub fn task_payload(mut self, value: Option<String>) -> Self {
        self.task_payload = Some(value);
        self
    }

    pub fn trigger(mut self, value: String) -> Self {
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

impl Default for IncidentRuleDataAttributesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentRuleDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRuleDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentRuleDataAttributesResponseVisitor {
            type Value = IncidentRuleDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<crate::datadogV2::model::IncidentRuleQueryCondition> =
                    None;
                let mut condition_table_type: Option<i32> = None;
                let mut conditions: Option<Vec<crate::datadogV2::model::IncidentRuleCondition>> =
                    None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by_uuid: Option<uuid::Uuid> = None;
                let mut deleted: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut enabled: Option<bool> = None;
                let mut execution_type: Option<i32> = None;
                let mut incident_settings_association_uuid: Option<Option<uuid::Uuid>> = None;
                let mut match_any_condition: Option<bool> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by_uuid: Option<uuid::Uuid> = None;
                let mut org_id: Option<i64> = None;
                let mut task_id: Option<Option<String>> = None;
                let mut task_payload: Option<Option<String>> = None;
                let mut trigger: Option<String> = None;
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
                        "condition_table_type" => {
                            if v.is_null() {
                                continue;
                            }
                            condition_table_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditions" => {
                            if v.is_null() {
                                continue;
                            }
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted" => {
                            deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_type" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_settings_association_uuid" => {
                            incident_settings_association_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_any_condition" => {
                            if v.is_null() {
                                continue;
                            }
                            match_any_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "task_id" => {
                            task_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentRuleDataAttributesResponse {
                    condition,
                    condition_table_type,
                    conditions,
                    created,
                    created_by_uuid,
                    deleted,
                    enabled,
                    execution_type,
                    incident_settings_association_uuid,
                    match_any_condition,
                    modified,
                    modified_by_uuid,
                    org_id,
                    task_id,
                    task_payload,
                    trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRuleDataAttributesResponseVisitor)
    }
}
