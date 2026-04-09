// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes containing the list of annotated interactions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsAnnotatedInteractionsDataAttributesResponse {
    /// List of interactions with their annotations.
    #[serde(rename = "annotated_interactions")]
    pub annotated_interactions: Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsAnnotatedInteractionsDataAttributesResponse {
    pub fn new(
        annotated_interactions: Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionItem>,
    ) -> LLMObsAnnotatedInteractionsDataAttributesResponse {
        LLMObsAnnotatedInteractionsDataAttributesResponse {
            annotated_interactions,
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

impl<'de> Deserialize<'de> for LLMObsAnnotatedInteractionsDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsAnnotatedInteractionsDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsAnnotatedInteractionsDataAttributesResponseVisitor {
            type Value = LLMObsAnnotatedInteractionsDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut annotated_interactions: Option<
                    Vec<crate::datadogV2::model::LLMObsAnnotatedInteractionItem>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "annotated_interactions" => {
                            annotated_interactions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let annotated_interactions = annotated_interactions
                    .ok_or_else(|| M::Error::missing_field("annotated_interactions"))?;

                let content = LLMObsAnnotatedInteractionsDataAttributesResponse {
                    annotated_interactions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsAnnotatedInteractionsDataAttributesResponseVisitor)
    }
}
