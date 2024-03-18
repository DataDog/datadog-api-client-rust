// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The data for a notebook.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookResponseData {
    /// The attributes of a notebook.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV1::model::NotebookResponseDataAttributes,
    /// Unique notebook ID, assigned when you create the notebook.
    #[serde(rename = "id")]
    pub id: i64,
    /// Type of the Notebook resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookResourceType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookResponseData {
    pub fn new(
        attributes: crate::datadogV1::model::NotebookResponseDataAttributes,
        id: i64,
        type_: crate::datadogV1::model::NotebookResourceType,
    ) -> NotebookResponseData {
        NotebookResponseData {
            attributes,
            id,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookResponseDataVisitor;
        impl<'a> Visitor<'a> for NotebookResponseDataVisitor {
            type Value = NotebookResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::NotebookResponseDataAttributes,
                > = None;
                let mut id: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::NotebookResourceType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::NotebookResourceType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = NotebookResponseData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookResponseDataVisitor)
    }
}
