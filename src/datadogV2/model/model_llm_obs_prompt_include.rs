// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An explicitly versioned prompt included as chat items. Omitting `items` includes every child message in its original order. When `items` is present, its zero-based indexes are inserted in the order provided; duplicate indexes are preserved.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPromptInclude {
    /// Optional ordered zero-based child-message indexes. Order and duplicate indexes are preserved.
    #[serde(rename = "items")]
    pub items: Option<Vec<i64>>,
    /// Customer-provided identifier of the included prompt. It cannot contain spaces, tabs, line breaks, braces, an equals sign, a comma, or quotes.
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    /// Positive sequential version number of the included prompt.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPromptInclude {
    pub fn new(prompt_id: String, version: i64) -> LLMObsPromptInclude {
        LLMObsPromptInclude {
            items: None,
            prompt_id,
            version,
            _unparsed: false,
        }
    }

    pub fn items(mut self, value: Vec<i64>) -> Self {
        self.items = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LLMObsPromptInclude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPromptIncludeVisitor;
        impl<'a> Visitor<'a> for LLMObsPromptIncludeVisitor {
            type Value = LLMObsPromptInclude;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut items: Option<Vec<i64>> = None;
                let mut prompt_id: Option<String> = None;
                let mut version: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "items" => {
                            if v.is_null() {
                                continue;
                            }
                            items = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_id" => {
                            prompt_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let prompt_id = prompt_id.ok_or_else(|| M::Error::missing_field("prompt_id"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = LLMObsPromptInclude {
                    items,
                    prompt_id,
                    version,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPromptIncludeVisitor)
    }
}
