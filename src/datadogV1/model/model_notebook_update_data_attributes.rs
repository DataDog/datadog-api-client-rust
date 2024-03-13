// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The data attributes of a notebook.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookUpdateDataAttributes {
    /// List of cells to display in the notebook.
    #[serde(rename = "cells")]
    pub cells: Vec<crate::datadogV1::model::NotebookUpdateCell>,
    /// Metadata associated with the notebook.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::NotebookMetadata>,
    /// The name of the notebook.
    #[serde(rename = "name")]
    pub name: String,
    /// Publication status of the notebook. For now, always "published".
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::NotebookStatus>,
    /// Notebook global timeframe.
    #[serde(rename = "time")]
    pub time: crate::datadogV1::model::NotebookGlobalTime,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookUpdateDataAttributes {
    pub fn new(
        cells: Vec<crate::datadogV1::model::NotebookUpdateCell>,
        name: String,
        time: crate::datadogV1::model::NotebookGlobalTime,
    ) -> NotebookUpdateDataAttributes {
        NotebookUpdateDataAttributes {
            cells,
            metadata: None,
            name,
            status: None,
            time,
            _unparsed: false,
        }
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::NotebookMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::NotebookStatus) -> Self {
        self.status = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for NotebookUpdateDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookUpdateDataAttributesVisitor;
        impl<'a> Visitor<'a> for NotebookUpdateDataAttributesVisitor {
            type Value = NotebookUpdateDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cells: Option<Vec<crate::datadogV1::model::NotebookUpdateCell>> = None;
                let mut metadata: Option<crate::datadogV1::model::NotebookMetadata> = None;
                let mut name: Option<String> = None;
                let mut status: Option<crate::datadogV1::model::NotebookStatus> = None;
                let mut time: Option<crate::datadogV1::model::NotebookGlobalTime> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cells" => {
                            cells = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::NotebookStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "time" => {
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    crate::datadogV1::model::NotebookGlobalTime::UnparsedObject(
                                        _time,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let cells = cells.ok_or_else(|| M::Error::missing_field("cells"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let time = time.ok_or_else(|| M::Error::missing_field("time"))?;

                let content = NotebookUpdateDataAttributes {
                    cells,
                    metadata,
                    name,
                    status,
                    time,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookUpdateDataAttributesVisitor)
    }
}
