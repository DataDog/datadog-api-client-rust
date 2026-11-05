// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A suggestion to tune the detection rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationRuleTuningSuggestion {
    /// Kind of change the suggestion makes to the rule.
    #[serde(rename = "action_type")]
    pub action_type: String,
    /// The conditions the change applies to.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningCondition>,
    /// What the suggestion changes.
    #[serde(rename = "description")]
    pub description: String,
    /// Estimated number of signals the change would have silenced in the window.
    #[serde(rename = "estimated_silenced_count")]
    pub estimated_silenced_count: Option<i64>,
    /// ID of the suggestion.
    #[serde(rename = "id")]
    pub id: String,
    /// How the conditions combine, such as AND or OR.
    #[serde(rename = "match_logic")]
    pub match_logic: String,
    /// Whether the suggestion is recommended.
    #[serde(rename = "recommended")]
    pub recommended: bool,
    /// ID of the detection rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// Title of the suggestion.
    #[serde(rename = "title")]
    pub title: String,
    /// What the change could hide. It can be empty.
    #[serde(rename = "warning")]
    pub warning: String,
    /// Why the suggestion is made.
    #[serde(rename = "why")]
    pub why: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationRuleTuningSuggestion {
    pub fn new(
        action_type: String,
        conditions: Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningCondition>,
        description: String,
        id: String,
        match_logic: String,
        recommended: bool,
        rule_id: String,
        title: String,
        warning: String,
        why: String,
    ) -> SecurityAgentInvestigationRuleTuningSuggestion {
        SecurityAgentInvestigationRuleTuningSuggestion {
            action_type,
            conditions,
            description,
            estimated_silenced_count: None,
            id,
            match_logic,
            recommended,
            rule_id,
            title,
            warning,
            why,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn estimated_silenced_count(mut self, value: i64) -> Self {
        self.estimated_silenced_count = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationRuleTuningSuggestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationRuleTuningSuggestionVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationRuleTuningSuggestionVisitor {
            type Value = SecurityAgentInvestigationRuleTuningSuggestion;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_type: Option<String> = None;
                let mut conditions: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningCondition>,
                > = None;
                let mut description: Option<String> = None;
                let mut estimated_silenced_count: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut match_logic: Option<String> = None;
                let mut recommended: Option<bool> = None;
                let mut rule_id: Option<String> = None;
                let mut title: Option<String> = None;
                let mut warning: Option<String> = None;
                let mut why: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_type" => {
                            action_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditions" => {
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "estimated_silenced_count" => {
                            if v.is_null() {
                                continue;
                            }
                            estimated_silenced_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_logic" => {
                            match_logic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended" => {
                            recommended =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warning" => {
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "why" => {
                            why = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action_type =
                    action_type.ok_or_else(|| M::Error::missing_field("action_type"))?;
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let match_logic =
                    match_logic.ok_or_else(|| M::Error::missing_field("match_logic"))?;
                let recommended =
                    recommended.ok_or_else(|| M::Error::missing_field("recommended"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;
                let warning = warning.ok_or_else(|| M::Error::missing_field("warning"))?;
                let why = why.ok_or_else(|| M::Error::missing_field("why"))?;

                let content = SecurityAgentInvestigationRuleTuningSuggestion {
                    action_type,
                    conditions,
                    description,
                    estimated_silenced_count,
                    id,
                    match_logic,
                    recommended,
                    rule_id,
                    title,
                    warning,
                    why,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationRuleTuningSuggestionVisitor)
    }
}
