// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with the notebook.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookMetadata {
    /// Whether or not the notebook is a template.
    #[serde(rename = "is_template")]
    pub is_template: Option<bool>,
    /// Whether or not the notebook takes snapshot image backups of the notebook's fixed-time graphs.
    #[serde(rename = "take_snapshots")]
    pub take_snapshots: Option<bool>,
    /// Metadata type of the notebook.
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option")]
    pub type_: Option<Option<crate::datadogV1::model::NotebookMetadataType>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookMetadata {
    pub fn new() -> NotebookMetadata {
        NotebookMetadata {
            is_template: None,
            take_snapshots: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn is_template(mut self, value: bool) -> Self {
        self.is_template = Some(value);
        self
    }

    pub fn take_snapshots(mut self, value: bool) -> Self {
        self.take_snapshots = Some(value);
        self
    }

    pub fn type_(mut self, value: Option<crate::datadogV1::model::NotebookMetadataType>) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for NotebookMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NotebookMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookMetadataVisitor;
        impl<'a> Visitor<'a> for NotebookMetadataVisitor {
            type Value = NotebookMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_template: Option<bool> = None;
                let mut take_snapshots: Option<bool> = None;
                let mut type_: Option<Option<crate::datadogV1::model::NotebookMetadataType>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_template" => {
                            if v.is_null() {
                                continue;
                            }
                            is_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "take_snapshots" => {
                            if v.is_null() {
                                continue;
                            }
                            take_snapshots =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    Some(crate::datadogV1::model::NotebookMetadataType::UnparsedObject(_type_)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = NotebookMetadata {
                    is_template,
                    take_snapshots,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookMetadataVisitor)
    }
}
