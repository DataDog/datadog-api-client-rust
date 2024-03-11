// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a notebook `markdown` cell.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookMarkdownCellAttributes {
    /// Text in a notebook is formatted with [Markdown](<https://daringfireball.net/projects/markdown/>), which enables the use of headings, subheadings, links, images, lists, and code blocks.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::NotebookMarkdownCellDefinition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookMarkdownCellAttributes {
    pub fn new(
        definition: crate::datadogV1::model::NotebookMarkdownCellDefinition,
    ) -> NotebookMarkdownCellAttributes {
        NotebookMarkdownCellAttributes {
            definition,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookMarkdownCellAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookMarkdownCellAttributesVisitor;
        impl<'a> Visitor<'a> for NotebookMarkdownCellAttributesVisitor {
            type Value = NotebookMarkdownCellAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<
                    crate::datadogV1::model::NotebookMarkdownCellDefinition,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;

                let content = NotebookMarkdownCellAttributes {
                    definition,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookMarkdownCellAttributesVisitor)
    }
}
