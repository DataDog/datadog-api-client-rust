// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an AI-generated incident postmortem.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentAIPostmortemDataAttributesResponse {
    /// Action items to prevent recurrence.
    #[serde(rename = "action_items")]
    pub action_items: Option<String>,
    /// The impact of the incident on customers.
    #[serde(rename = "customer_impact")]
    pub customer_impact: Option<String>,
    /// An executive summary of the incident.
    #[serde(rename = "executive_summary")]
    pub executive_summary: Option<String>,
    /// Key insights from the incident.
    #[serde(rename = "key_insights")]
    pub key_insights: Option<String>,
    /// Key timeline events during the incident.
    #[serde(rename = "key_timeline")]
    pub key_timeline: Option<String>,
    /// Lessons learned from the incident.
    #[serde(rename = "lessons_learned")]
    pub lessons_learned: Option<String>,
    /// An overview of the affected systems.
    #[serde(rename = "system_overview")]
    pub system_overview: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentAIPostmortemDataAttributesResponse {
    pub fn new() -> IncidentAIPostmortemDataAttributesResponse {
        IncidentAIPostmortemDataAttributesResponse {
            action_items: None,
            customer_impact: None,
            executive_summary: None,
            key_insights: None,
            key_timeline: None,
            lessons_learned: None,
            system_overview: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action_items(mut self, value: String) -> Self {
        self.action_items = Some(value);
        self
    }

    pub fn customer_impact(mut self, value: String) -> Self {
        self.customer_impact = Some(value);
        self
    }

    pub fn executive_summary(mut self, value: String) -> Self {
        self.executive_summary = Some(value);
        self
    }

    pub fn key_insights(mut self, value: String) -> Self {
        self.key_insights = Some(value);
        self
    }

    pub fn key_timeline(mut self, value: String) -> Self {
        self.key_timeline = Some(value);
        self
    }

    pub fn lessons_learned(mut self, value: String) -> Self {
        self.lessons_learned = Some(value);
        self
    }

    pub fn system_overview(mut self, value: String) -> Self {
        self.system_overview = Some(value);
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

impl Default for IncidentAIPostmortemDataAttributesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentAIPostmortemDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentAIPostmortemDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentAIPostmortemDataAttributesResponseVisitor {
            type Value = IncidentAIPostmortemDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_items: Option<String> = None;
                let mut customer_impact: Option<String> = None;
                let mut executive_summary: Option<String> = None;
                let mut key_insights: Option<String> = None;
                let mut key_timeline: Option<String> = None;
                let mut lessons_learned: Option<String> = None;
                let mut system_overview: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_items" => {
                            if v.is_null() {
                                continue;
                            }
                            action_items =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impact" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_impact =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "executive_summary" => {
                            if v.is_null() {
                                continue;
                            }
                            executive_summary =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_insights" => {
                            if v.is_null() {
                                continue;
                            }
                            key_insights =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_timeline" => {
                            if v.is_null() {
                                continue;
                            }
                            key_timeline =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lessons_learned" => {
                            if v.is_null() {
                                continue;
                            }
                            lessons_learned =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "system_overview" => {
                            if v.is_null() {
                                continue;
                            }
                            system_overview =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentAIPostmortemDataAttributesResponse {
                    action_items,
                    customer_impact,
                    executive_summary,
                    key_insights,
                    key_timeline,
                    lessons_learned,
                    system_overview,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentAIPostmortemDataAttributesResponseVisitor)
    }
}
