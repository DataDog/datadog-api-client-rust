// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes containing an annotated interaction and its related events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotatedInteractionDataAttributesResponse {
    /// An interaction with its associated annotations.
    #[serde(rename = "annotated_interaction")]
    pub annotated_interaction: crate::datadogV2::model::LLMObsAnnotatedInteractionItem,
    /// Page of events associated with the annotated interaction.
    #[serde(rename = "events")]
    pub events: Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionEvent>,
    /// Type of an annotated interaction.
    #[serde(rename = "interaction_type")]
    pub interaction_type: crate::datadogV2::model::LLMObsAnyInteractionType,
    /// Cursor to retrieve the next page of events. Absent when there are no more events.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotatedInteractionDataAttributesResponse {
    pub fn new(
        annotated_interaction: crate::datadogV2::model::LLMObsAnnotatedInteractionItem,
        events: Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionEvent>,
        interaction_type: crate::datadogV2::model::LLMObsAnyInteractionType,
    ) -> LLMObsAnnotatedInteractionDataAttributesResponse {
        LLMObsAnnotatedInteractionDataAttributesResponse {
            annotated_interaction,
            events,
            interaction_type,
            next_cursor: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_cursor(mut self, value: String) -> Self {
        self.next_cursor = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsAnnotatedInteractionDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotatedInteractionDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotatedInteractionDataAttributesResponseVisitor {
            type Value = LLMObsAnnotatedInteractionDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotated_interaction: Option<
                    crate::datadogV2::model::LLMObsAnnotatedInteractionItem,
                > = None;
                let mut events: Option<
                    Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionEvent>,
                > = None;
                let mut interaction_type: Option<
                    crate::datadogV2::model::LLMObsAnyInteractionType,
                > = None;
                let mut next_cursor: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotated_interaction" => {
                            annotated_interaction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _annotated_interaction) = annotated_interaction {
                                match _annotated_interaction {
                                    crate::datadogV2::model::LLMObsAnnotatedInteractionItem::UnparsedObject(_annotated_interaction) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "events" => {
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interaction_type" => {
                            interaction_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _interaction_type) = interaction_type {
                                match _interaction_type {
                                    crate::datadogV2::model::LLMObsAnyInteractionType::UnparsedObject(_interaction_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "next_cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            next_cursor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotated_interaction = annotated_interaction
                    .ok_or_else(|| M::Error::missing_field("annotated_interaction"))?;
                let events = events.ok_or_else(|| M::Error::missing_field("events"))?;
                let interaction_type =
                    interaction_type.ok_or_else(|| M::Error::missing_field("interaction_type"))?;

                let content = LLMObsAnnotatedInteractionDataAttributesResponse {
                    annotated_interaction,
                    events,
                    interaction_type,
                    next_cursor,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotatedInteractionDataAttributesResponseVisitor)
    }
}
