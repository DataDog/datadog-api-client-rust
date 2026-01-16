// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Postmortem attachment attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PostmortemAttachmentRequestAttributes {
    /// The cells of the postmortem
    #[serde(rename = "cells")]
    pub cells: Option<Vec<crate::datadogV2::model::PostmortemCell>>,
    /// The content of the postmortem
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// The ID of the postmortem template
    #[serde(rename = "postmortem_template_id")]
    pub postmortem_template_id: Option<String>,
    /// The title of the postmortem
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PostmortemAttachmentRequestAttributes {
    pub fn new() -> PostmortemAttachmentRequestAttributes {
        PostmortemAttachmentRequestAttributes {
            cells: None,
            content: None,
            postmortem_template_id: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cells(mut self, value: Vec<crate::datadogV2::model::PostmortemCell>) -> Self {
        self.cells = Some(value);
        self
    }

    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    pub fn postmortem_template_id(mut self, value: String) -> Self {
        self.postmortem_template_id = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for PostmortemAttachmentRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PostmortemAttachmentRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PostmortemAttachmentRequestAttributesVisitor;
        impl<'a> Visitor<'a> for PostmortemAttachmentRequestAttributesVisitor {
            type Value = PostmortemAttachmentRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cells: Option<Vec<crate::datadogV2::model::PostmortemCell>> = None;
                let mut content: Option<String> = None;
                let mut postmortem_template_id: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cells" => {
                            if v.is_null() {
                                continue;
                            }
                            cells = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "postmortem_template_id" => {
                            if v.is_null() {
                                continue;
                            }
                            postmortem_template_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PostmortemAttachmentRequestAttributes {
                    cells,
                    content,
                    postmortem_template_id,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PostmortemAttachmentRequestAttributesVisitor)
    }
}
