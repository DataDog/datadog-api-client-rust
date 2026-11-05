// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A suggested mitigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationMitigationSuggestion {
    /// Actions that apply the mitigation.
    #[serde(rename = "actions")]
    pub actions: Option<Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationAction>>,
    /// ID of the mitigation.
    #[serde(rename = "id")]
    pub id: String,
    /// Why the mitigation is suggested.
    #[serde(rename = "rationale")]
    pub rationale: String,
    /// Whether the mitigation is recommended.
    #[serde(rename = "recommended")]
    pub recommended: bool,
    /// What the mitigation applies to.
    #[serde(rename = "scope_note")]
    pub scope_note: Option<String>,
    /// Order in which to apply the mitigation, starting at 1.
    #[serde(rename = "step")]
    pub step: i64,
    /// Title of the mitigation.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationMitigationSuggestion {
    pub fn new(
        id: String,
        rationale: String,
        recommended: bool,
        step: i64,
        title: String,
    ) -> SecurityAgentInvestigationMitigationSuggestion {
        SecurityAgentInvestigationMitigationSuggestion {
            actions: None,
            id,
            rationale,
            recommended,
            scope_note: None,
            step,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actions(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationAction>,
    ) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn scope_note(mut self, value: String) -> Self {
        self.scope_note = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationMitigationSuggestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationMitigationSuggestionVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationMitigationSuggestionVisitor {
            type Value = SecurityAgentInvestigationMitigationSuggestion;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions: Option<
                    Vec<crate::datadogV2::model::SecurityAgentInvestigationMitigationAction>,
                > = None;
                let mut id: Option<String> = None;
                let mut rationale: Option<String> = None;
                let mut recommended: Option<bool> = None;
                let mut scope_note: Option<String> = None;
                let mut step: Option<i64> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions" => {
                            if v.is_null() {
                                continue;
                            }
                            actions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rationale" => {
                            rationale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended" => {
                            recommended =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_note" => {
                            if v.is_null() {
                                continue;
                            }
                            scope_note = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "step" => {
                            step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let rationale = rationale.ok_or_else(|| M::Error::missing_field("rationale"))?;
                let recommended =
                    recommended.ok_or_else(|| M::Error::missing_field("recommended"))?;
                let step = step.ok_or_else(|| M::Error::missing_field("step"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = SecurityAgentInvestigationMitigationSuggestion {
                    actions,
                    id,
                    rationale,
                    recommended,
                    scope_note,
                    step,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationMitigationSuggestionVisitor)
    }
}
