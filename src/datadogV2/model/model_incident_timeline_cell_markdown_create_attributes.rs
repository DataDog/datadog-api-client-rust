// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Timeline cell data for Markdown timeline cells for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTimelineCellMarkdownCreateAttributes {
    /// Type of the Markdown timeline cell.
    #[serde(rename = "cell_type")]
    pub cell_type: crate::datadogV2::model::IncidentTimelineCellMarkdownContentType,
    /// The Markdown timeline cell contents.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributesContent,
    /// A flag indicating whether the timeline cell is important and should be highlighted.
    #[serde(rename = "important")]
    pub important: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTimelineCellMarkdownCreateAttributes {
    pub fn new(
        cell_type: crate::datadogV2::model::IncidentTimelineCellMarkdownContentType,
        content: crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributesContent,
    ) -> IncidentTimelineCellMarkdownCreateAttributes {
        IncidentTimelineCellMarkdownCreateAttributes {
            cell_type,
            content,
            important: None,
            _unparsed: false,
        }
    }

    pub fn important(mut self, value: bool) -> Self {
        self.important = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentTimelineCellMarkdownCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTimelineCellMarkdownCreateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentTimelineCellMarkdownCreateAttributesVisitor {
            type Value = IncidentTimelineCellMarkdownCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cell_type: Option<
                    crate::datadogV2::model::IncidentTimelineCellMarkdownContentType,
                > = None;
                let mut content: Option<
                    crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributesContent,
                > = None;
                let mut important: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cell_type" => {
                            cell_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_type) = cell_type {
                                match _cell_type {
                                    crate::datadogV2::model::IncidentTimelineCellMarkdownContentType::UnparsedObject(_cell_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "important" => {
                            if v.is_null() {
                                continue;
                            }
                            important = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let cell_type = cell_type.ok_or_else(|| M::Error::missing_field("cell_type"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;

                let content = IncidentTimelineCellMarkdownCreateAttributes {
                    cell_type,
                    content,
                    important,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTimelineCellMarkdownCreateAttributesVisitor)
    }
}
