// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An explicitly versioned whole-chat or selective-message include.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPromptIncludeItem {
    /// An explicitly versioned prompt included as chat items. Omitting `items` includes every child message in its original order. When `items` is present, its zero-based indexes are inserted in the order provided; duplicate indexes are preserved.
    #[serde(rename = "include")]
    pub include: crate::datadogV2::model::LLMObsPromptInclude,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPromptIncludeItem {
    pub fn new(include: crate::datadogV2::model::LLMObsPromptInclude) -> LLMObsPromptIncludeItem {
        LLMObsPromptIncludeItem {
            include,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for LLMObsPromptIncludeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPromptIncludeItemVisitor;
        impl<'a> Visitor<'a> for LLMObsPromptIncludeItemVisitor {
            type Value = LLMObsPromptIncludeItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include: Option<crate::datadogV2::model::LLMObsPromptInclude> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;

                let content = LLMObsPromptIncludeItem { include, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPromptIncludeItemVisitor)
    }
}
