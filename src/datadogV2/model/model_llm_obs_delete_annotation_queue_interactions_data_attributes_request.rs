// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for deleting interactions from an annotation queue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
    /// List of interaction IDs to delete. Must contain at least one item.
    #[serde(rename = "interaction_ids")]
    pub interaction_ids: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
    pub fn new(
        interaction_ids: Vec<String>,
    ) -> LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
        LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
            interaction_ids,
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

impl<'de> Deserialize<'de> for LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequestVisitor {
            type Value = LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interaction_ids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interaction_ids" => {
                            interaction_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let interaction_ids =
                    interaction_ids.ok_or_else(|| M::Error::missing_field("interaction_ids"))?;

                let content = LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequest {
                    interaction_ids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(LLMObsDeleteAnnotationQueueInteractionsDataAttributesRequestVisitor)
    }
}
