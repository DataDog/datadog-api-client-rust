// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for the action to execute, dependent on the action type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutomationRuleActionData {
    /// The type of AI agent to assign. Required when the action type is `assign_agent`.
    #[serde(rename = "agent_type")]
    pub agent_type: Option<String>,
    /// The identifier of the AI agent to assign to the case. Required when the action type is `assign_agent`.
    #[serde(rename = "assigned_agent_id")]
    pub assigned_agent_id: Option<String>,
    /// The handle of the Datadog workflow to execute. Required when the action type is `execute_workflow`.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutomationRuleActionData {
    pub fn new() -> AutomationRuleActionData {
        AutomationRuleActionData {
            agent_type: None,
            assigned_agent_id: None,
            handle: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn agent_type(mut self, value: String) -> Self {
        self.agent_type = Some(value);
        self
    }

    pub fn assigned_agent_id(mut self, value: String) -> Self {
        self.assigned_agent_id = Some(value);
        self
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
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

impl Default for AutomationRuleActionData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AutomationRuleActionData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutomationRuleActionDataVisitor;
        impl<'a> Visitor<'a> for AutomationRuleActionDataVisitor {
            type Value = AutomationRuleActionData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_type: Option<String> = None;
                let mut assigned_agent_id: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_type" => {
                            if v.is_null() {
                                continue;
                            }
                            agent_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assigned_agent_id" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_agent_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AutomationRuleActionData {
                    agent_type,
                    assigned_agent_id,
                    handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutomationRuleActionDataVisitor)
    }
}
