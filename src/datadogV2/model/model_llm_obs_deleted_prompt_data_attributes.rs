// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes confirming that an LLM Observability prompt was deleted.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDeletedPromptDataAttributes {
    /// Timestamp when the prompt was deleted.
    #[serde(rename = "deleted_at")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    /// Customer-provided identifier of the deleted prompt.
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDeletedPromptDataAttributes {
    pub fn new(
        deleted_at: chrono::DateTime<chrono::Utc>,
        prompt_id: String,
    ) -> LLMObsDeletedPromptDataAttributes {
        LLMObsDeletedPromptDataAttributes {
            deleted_at,
            prompt_id,
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

impl<'de> Deserialize<'de> for LLMObsDeletedPromptDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDeletedPromptDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsDeletedPromptDataAttributesVisitor {
            type Value = LLMObsDeletedPromptDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut prompt_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_id" => {
                            prompt_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let deleted_at = deleted_at.ok_or_else(|| M::Error::missing_field("deleted_at"))?;
                let prompt_id = prompt_id.ok_or_else(|| M::Error::missing_field("prompt_id"))?;

                let content = LLMObsDeletedPromptDataAttributes {
                    deleted_at,
                    prompt_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDeletedPromptDataAttributesVisitor)
    }
}
