// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// timeline cell
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimelineCell {
    /// author of the timeline cell
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::TimelineCellAuthor>,
    /// timeline cell content
    #[serde(rename = "cell_content")]
    pub cell_content: Option<crate::datadogV2::model::TimelineCellContent>,
    /// Timestamp of when the cell was created
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the cell was deleted
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of when the cell was last modified
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timeline cell content type
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::TimelineCellType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimelineCell {
    pub fn new() -> TimelineCell {
        TimelineCell {
            author: None,
            cell_content: None,
            created_at: None,
            deleted_at: None,
            modified_at: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV2::model::TimelineCellAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn cell_content(mut self, value: crate::datadogV2::model::TimelineCellContent) -> Self {
        self.cell_content = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::TimelineCellType) -> Self {
        self.type_ = Some(value);
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

impl Default for TimelineCell {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimelineCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimelineCellVisitor;
        impl<'a> Visitor<'a> for TimelineCellVisitor {
            type Value = TimelineCell;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::TimelineCellAuthor> = None;
                let mut cell_content: Option<crate::datadogV2::model::TimelineCellContent> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut type_: Option<crate::datadogV2::model::TimelineCellType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _author) = author {
                                match _author {
                                    crate::datadogV2::model::TimelineCellAuthor::UnparsedObject(
                                        _author,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "cell_content" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cell_content) = cell_content {
                                match _cell_content {
                                    crate::datadogV2::model::TimelineCellContent::UnparsedObject(_cell_content) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TimelineCellType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TimelineCell {
                    author,
                    cell_content,
                    created_at,
                    deleted_at,
                    modified_at,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimelineCellVisitor)
    }
}
