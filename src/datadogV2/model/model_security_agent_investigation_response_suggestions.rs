// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Suggested responses to the signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationResponseSuggestions {
    /// Suggested mitigations, in the order to apply them.
    #[serde(rename = "mitigation_suggestions")]
    pub mitigation_suggestions:
        Option<Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationSuggestion>>,
    /// Suggestions to tune the detection rule that generated the signal.
    #[serde(rename = "rule_tuning_suggestions")]
    pub rule_tuning_suggestions:
        Option<crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestions>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationResponseSuggestions {
    pub fn new() -> SecurityAgentInvestigationResponseSuggestions {
        SecurityAgentInvestigationResponseSuggestions {
            mitigation_suggestions: None,
            rule_tuning_suggestions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn mitigation_suggestions(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationSuggestion>,
    ) -> Self {
        self.mitigation_suggestions = Some(value);
        self
    }

    pub fn rule_tuning_suggestions(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestions,
    ) -> Self {
        self.rule_tuning_suggestions = Some(value);
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

impl Default for SecurityAgentInvestigationResponseSuggestions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityAgentInvestigationResponseSuggestions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationResponseSuggestionsVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationResponseSuggestionsVisitor {
            type Value = SecurityAgentInvestigationResponseSuggestions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mitigation_suggestions: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationSuggestion>,
                > = None;
                let mut rule_tuning_suggestions: Option<
                    crate::datadogV2::model::SecurityAgentInvestigationRuleTuningSuggestions,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mitigation_suggestions" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigation_suggestions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_tuning_suggestions" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_tuning_suggestions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityAgentInvestigationResponseSuggestions {
                    mitigation_suggestions,
                    rule_tuning_suggestions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationResponseSuggestionsVisitor)
    }
}
