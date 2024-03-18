// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Text in a notebook is formatted with [Markdown](<https://daringfireball.net/projects/markdown/>), which enables the use of headings, subheadings, links, images, lists, and code blocks.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookMarkdownCellDefinition {
    /// The markdown content.
    #[serde(rename = "text")]
    pub text: String,
    /// Type of the markdown cell.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookMarkdownCellDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookMarkdownCellDefinition {
    pub fn new(
        text: String,
        type_: crate::datadogV1::model::NotebookMarkdownCellDefinitionType,
    ) -> NotebookMarkdownCellDefinition {
        NotebookMarkdownCellDefinition {
            text,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookMarkdownCellDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookMarkdownCellDefinitionVisitor;
        impl<'a> Visitor<'a> for NotebookMarkdownCellDefinitionVisitor {
            type Value = NotebookMarkdownCellDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut text: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::NotebookMarkdownCellDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "text" => {
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::NotebookMarkdownCellDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let text = text.ok_or_else(|| M::Error::missing_field("text"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NotebookMarkdownCellDefinition {
                    text,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookMarkdownCellDefinitionVisitor)
    }
}
