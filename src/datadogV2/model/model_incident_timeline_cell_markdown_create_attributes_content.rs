// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Markdown timeline cell contents.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimelineCellMarkdownCreateAttributesContent {
    /// The Markdown content of the cell.
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimelineCellMarkdownCreateAttributesContent {
    pub fn new() -> IncidentTimelineCellMarkdownCreateAttributesContent {
        IncidentTimelineCellMarkdownCreateAttributesContent {
            content: None,
            _unparsed: false,
        }
    }

    pub fn content(&mut self, value: String) -> &mut Self {
        self.content = Some(value);
        self
    }
}

impl Default for IncidentTimelineCellMarkdownCreateAttributesContent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentTimelineCellMarkdownCreateAttributesContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimelineCellMarkdownCreateAttributesContentVisitor;
        impl<'a> Visitor<'a> for IncidentTimelineCellMarkdownCreateAttributesContentVisitor {
            type Value = IncidentTimelineCellMarkdownCreateAttributesContent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content =
                    IncidentTimelineCellMarkdownCreateAttributesContent { content, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimelineCellMarkdownCreateAttributesContentVisitor)
    }
}
