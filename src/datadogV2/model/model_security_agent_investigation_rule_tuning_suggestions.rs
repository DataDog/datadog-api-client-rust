// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Suggestions to tune the detection rule that generated the signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationRuleTuningSuggestions {
    /// Length of the window, in days.
    #[serde(rename = "occurrence_window_days")]
    pub occurrence_window_days: i64,
    /// Why tuning the rule is suggested.
    #[serde(rename = "reasoning")]
    pub reasoning: String,
    /// ID of the detection rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// Name of the detection rule.
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    /// Number of signals the rule generated in the window.
    #[serde(rename = "rule_signal_total")]
    pub rule_signal_total: i64,
    /// The tuning suggestions.
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestion>,
    /// Summary of the suggestions.
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationRuleTuningSuggestions {
    pub fn new(
        occurrence_window_days: i64,
        reasoning: String,
        rule_id: String,
        rule_name: String,
        rule_signal_total: i64,
        suggestions: Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestion>,
        summary: String,
    ) -> SecurityAgentInvestigationRuleTuningSuggestions {
        SecurityAgentInvestigationRuleTuningSuggestions {
            occurrence_window_days,
            reasoning,
            rule_id,
            rule_name,
            rule_signal_total,
            suggestions,
            summary,
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationRuleTuningSuggestions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationRuleTuningSuggestionsVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationRuleTuningSuggestionsVisitor {
            type Value = SecurityAgentInvestigationRuleTuningSuggestions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut occurrence_window_days: Option<i64> = None;
                let mut reasoning: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut rule_name: Option<String> = None;
                let mut rule_signal_total: Option<i64> = None;
                let mut suggestions: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestion>,
                > = None;
                let mut summary: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "occurrence_window_days" => {
                            occurrence_window_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reasoning" => {
                            reasoning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_id" => {
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_name" => {
                            rule_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_signal_total" => {
                            rule_signal_total =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suggestions" => {
                            suggestions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary" => {
                            summary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let occurrence_window_days = occurrence_window_days
                    .ok_or_else(|| M::Error::missing_field("occurrence_window_days"))?;
                let reasoning = reasoning.ok_or_else(|| M::Error::missing_field("reasoning"))?;
                let rule_id = rule_id.ok_or_else(|| M::Error::missing_field("rule_id"))?;
                let rule_name = rule_name.ok_or_else(|| M::Error::missing_field("rule_name"))?;
                let rule_signal_total = rule_signal_total
                    .ok_or_else(|| M::Error::missing_field("rule_signal_total"))?;
                let suggestions =
                    suggestions.ok_or_else(|| M::Error::missing_field("suggestions"))?;
                let summary = summary.ok_or_else(|| M::Error::missing_field("summary"))?;

                let content = SecurityAgentInvestigationRuleTuningSuggestions {
                    occurrence_window_days,
                    reasoning,
                    rule_id,
                    rule_name,
                    rule_signal_total,
                    suggestions,
                    summary,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationRuleTuningSuggestionsVisitor)
    }
}
