// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a ticket creation rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TicketCreationRuleAttributesCreate {
    /// The action to take when the ticket creation rule matches a finding.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::TicketCreationRuleAction,
    /// Whether the ticket creation rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The name of the ticket creation rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Defines the scope of findings to which the automation rule applies.
    #[serde(rename = "rule")]
    pub rule: crate::datadogV2::model::AutomationRuleScope,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TicketCreationRuleAttributesCreate {
    pub fn new(
        action: crate::datadogV2::model::TicketCreationRuleAction,
        name: String,
        rule: crate::datadogV2::model::AutomationRuleScope,
    ) -> TicketCreationRuleAttributesCreate {
        TicketCreationRuleAttributesCreate {
            action,
            enabled: None,
            name,
            rule,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

impl<'de> Deserialize<'de> for TicketCreationRuleAttributesCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TicketCreationRuleAttributesCreateVisitor;
        impl<'a> Visitor<'a> for TicketCreationRuleAttributesCreateVisitor {
            type Value = TicketCreationRuleAttributesCreate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::TicketCreationRuleAction> = None;
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut rule: Option<crate::datadogV2::model::AutomationRuleScope> = None;
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
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let rule = rule.ok_or_else(|| M::Error::missing_field("rule"))?;

                let content = TicketCreationRuleAttributesCreate {
                    action,
                    enabled,
                    name,
                    rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TicketCreationRuleAttributesCreateVisitor)
    }
}
