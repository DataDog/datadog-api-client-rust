// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Core attributes of an automation rule, including its name, trigger condition, action to execute, and current state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutomationRuleAttributes {
    /// Defines what happens when the rule triggers. Combines an action type with action-specific configuration data.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::AutomationRuleAction,
    /// Timestamp when the automation rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the automation rule was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A human-readable name for the automation rule, used to identify the rule in the UI and API responses.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the automation rule is active. Enabled rules trigger on matching case events; disabled rules are inactive but preserve their configuration.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::CaseAutomationRuleState,
    /// Defines when the rule activates. Combines a trigger type (the case event to listen for) with optional trigger data (conditions that narrow when the trigger fires).
    #[serde(rename = "trigger")]
    pub trigger: crate::datadogV2::model::AutomationRuleTrigger,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutomationRuleAttributes {
    pub fn new(
        action: crate::datadogV2::model::AutomationRuleAction,
        created_at: chrono::DateTime<chrono::Utc>,
        name: String,
        state: crate::datadogV2::model::CaseAutomationRuleState,
        trigger: crate::datadogV2::model::AutomationRuleTrigger,
    ) -> AutomationRuleAttributes {
        AutomationRuleAttributes {
            action,
            created_at,
            modified_at: None,
            name,
            state,
            trigger,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
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

impl<'de> Deserialize<'de> for AutomationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutomationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for AutomationRuleAttributesVisitor {
            type Value = AutomationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::AutomationRuleAction> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::CaseAutomationRuleState> = None;
                let mut trigger: Option<crate::datadogV2::model::AutomationRuleTrigger> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::CaseAutomationRuleState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "trigger" => {
                            trigger = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;
                let trigger = trigger.ok_or_else(|| M::Error::missing_field("trigger"))?;

                let content = AutomationRuleAttributes {
                    action,
                    created_at,
                    modified_at,
                    name,
                    state,
                    trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutomationRuleAttributesVisitor)
    }
}
