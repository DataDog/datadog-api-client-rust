// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the inbox rule
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InboxRuleAttributes {
    /// Action of the inbox rule
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::ActionInbox,
    /// Date as Unix timestamp in milliseconds
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// User creating or modifying a rule
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::RuleUser,
    /// Field used to enable or disable the rule.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Date as Unix timestamp in milliseconds
    #[serde(rename = "modified_at")]
    pub modified_at: i64,
    /// User creating or modifying a rule
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::RuleUser,
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

impl InboxRuleAttributes {
    pub fn new(
        action: crate::datadogV2::model::ActionInbox,
        created_at: i64,
        created_by: crate::datadogV2::model::RuleUser,
        enabled: bool,
        modified_at: i64,
        modified_by: crate::datadogV2::model::RuleUser,
        name: String,
        rule: crate::datadogV2::model::Rule,
    ) -> InboxRuleAttributes {
        InboxRuleAttributes {
            action,
            created_at,
            created_by,
            enabled,
            modified_at,
            modified_by,
            name,
            rule,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for InboxRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InboxRuleAttributesVisitor;
        impl<'a> Visitor<'a> for InboxRuleAttributesVisitor {
            type Value = InboxRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::ActionInbox> = None;
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<crate::datadogV2::model::RuleUser> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<i64> = None;
                let mut modified_by: Option<crate::datadogV2::model::RuleUser> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rule = rule.ok_or_else(|| M::Error::missing_field("rule"))?;

                let content = InboxRuleAttributes {
                    action,
                    created_at,
                    created_by,
                    enabled,
                    modified_at,
                    modified_by,
                    name,
                    rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(InboxRuleAttributesVisitor)
    }
}
