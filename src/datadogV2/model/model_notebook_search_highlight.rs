// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Highlighted fields from the notebook search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookSearchHighlight {
    /// Highlighted cell text matches.
    #[serde(rename = "cells.text")]
    pub cells_text: Option<Vec<String>>,
    /// Highlighted cell title matches.
    #[serde(rename = "cells.title")]
    pub cells_title: Option<Vec<String>>,
    /// Highlighted notebook name matches.
    #[serde(rename = "name")]
    pub name: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookSearchHighlight {
    pub fn new() -> NotebookSearchHighlight {
        NotebookSearchHighlight {
            cells_text: None,
            cells_title: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cells_text(mut self, value: Vec<String>) -> Self {
        self.cells_text = Some(value);
        self
    }

    pub fn cells_title(mut self, value: Vec<String>) -> Self {
        self.cells_title = Some(value);
        self
    }

    pub fn name(mut self, value: Vec<String>) -> Self {
        self.name = Some(value);
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

impl Default for NotebookSearchHighlight {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NotebookSearchHighlight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookSearchHighlightVisitor;
        impl<'a> Visitor<'a> for NotebookSearchHighlightVisitor {
            type Value = NotebookSearchHighlight;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cells_text: Option<Vec<String>> = None;
                let mut cells_title: Option<Vec<String>> = None;
                let mut name: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cells.text" => {
                            if v.is_null() {
                                continue;
                            }
                            cells_text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cells.title" => {
                            if v.is_null() {
                                continue;
                            }
                            cells_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NotebookSearchHighlight {
                    cells_text,
                    cells_title,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookSearchHighlightVisitor)
    }
}
