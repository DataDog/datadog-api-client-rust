// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the inbox rule create request: the rule name, the rule details, the associated action, and the optional enabled field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateInboxRuleParametersDataAttributes {
    /// Action of the inbox rule
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::ActionInbox,
    /// Field used to enable or disable the rule.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Name of the pipeline rule
    #[serde(rename = "name")]
    pub name: String,
    /// The definition of an automation pipeline rule scope.
    /// A rule can act on specific issue types, security rule types, security rule IDs, rule severities, or a query.
    /// The query can be used to filter resources on tags and attributes.
    /// The issue type and rule types fields are required.
    #[serde(rename = "rule")]
    pub rule: crate::datadogV2::model::Rule,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateInboxRuleParametersDataAttributes {
    pub fn new(
        action: crate::datadogV2::model::ActionInbox,
        name: String,
        rule: crate::datadogV2::model::Rule,
    ) -> CreateInboxRuleParametersDataAttributes {
        CreateInboxRuleParametersDataAttributes {
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

impl<'de> Deserialize<'de> for CreateInboxRuleParametersDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateInboxRuleParametersDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateInboxRuleParametersDataAttributesVisitor {
            type Value = CreateInboxRuleParametersDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::ActionInbox> = None;
                let mut enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut rule: Option<crate::datadogV2::model::Rule> = None;
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

                let content = CreateInboxRuleParametersDataAttributes {
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

        deserializer.deserialize_any(CreateInboxRuleParametersDataAttributesVisitor)
    }
}
