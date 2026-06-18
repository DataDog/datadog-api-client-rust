// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a ticket creation rule returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TicketCreationRuleAttributesResponse {
    /// The action to take when the ticket creation rule matches a finding.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::TicketCreationRuleActionResponse,
    /// The Unix timestamp in milliseconds when the rule was created.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// The user or Datadog system who created the rule.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::AutomationRuleCreatedBy,
    /// Whether the ticket creation rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The Unix timestamp in milliseconds when the rule was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: i64,
    /// The user or Datadog system who last modified the rule.
    #[serde(rename = "modified_by")]
    pub modified_by: crate::datadogV2::model::AutomationRuleModifiedBy,
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

impl TicketCreationRuleAttributesResponse {
    pub fn new(
        action: crate::datadogV2::model::TicketCreationRuleActionResponse,
        created_at: i64,
        created_by: crate::datadogV2::model::AutomationRuleCreatedBy,
        enabled: bool,
        modified_at: i64,
        modified_by: crate::datadogV2::model::AutomationRuleModifiedBy,
        name: String,
        rule: crate::datadogV2::model::AutomationRuleScope,
    ) -> TicketCreationRuleAttributesResponse {
        TicketCreationRuleAttributesResponse {
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

impl<'de> Deserialize<'de> for TicketCreationRuleAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TicketCreationRuleAttributesResponseVisitor;
        impl<'a> Visitor<'a> for TicketCreationRuleAttributesResponseVisitor {
            type Value = TicketCreationRuleAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::TicketCreationRuleActionResponse> =
                    None;
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<crate::datadogV2::model::AutomationRuleCreatedBy> = None;
                let mut enabled: Option<bool> = None;
                let mut modified_at: Option<i64> = None;
                let mut modified_by: Option<crate::datadogV2::model::AutomationRuleModifiedBy> =
                    None;
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

                let content = TicketCreationRuleAttributesResponse {
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

        deserializer.deserialize_any(TicketCreationRuleAttributesResponseVisitor)
    }
}
