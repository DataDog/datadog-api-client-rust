// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes required to create an automation rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutomationRuleCreateAttributes {
    /// Defines what happens when the rule triggers. Combines an action type with action-specific configuration data.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::AutomationRuleAction,
    /// Name of the automation rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the automation rule is active. Enabled rules trigger on matching case events; disabled rules are inactive but preserve their configuration.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::CaseAutomationRuleState>,
    /// Defines when the rule activates. Combines a trigger type (the case event to listen for) with optional trigger data (conditions that narrow when the trigger fires).
    #[serde(rename = "trigger")]
    pub trigger: crate::datadogV2::model::AutomationRuleTrigger,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutomationRuleCreateAttributes {
    pub fn new(
        action: crate::datadogV2::model::AutomationRuleAction,
        name: String,
        trigger: crate::datadogV2::model::AutomationRuleTrigger,
    ) -> AutomationRuleCreateAttributes {
        AutomationRuleCreateAttributes {
            action,
            name,
            state: None,
            trigger,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn state(mut self, value: crate::datadogV2::model::CaseAutomationRuleState) -> Self {
        self.state = Some(value);
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

impl<'de> Deserialize<'de> for AutomationRuleCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutomationRuleCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AutomationRuleCreateAttributesVisitor {
            type Value = AutomationRuleCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::AutomationRuleAction> = None;
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
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let trigger = trigger.ok_or_else(|| M::Error::missing_field("trigger"))?;

                let content = AutomationRuleCreateAttributes {
                    action,
                    name,
                    state,
                    trigger,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutomationRuleCreateAttributesVisitor)
    }
}
