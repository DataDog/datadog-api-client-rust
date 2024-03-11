// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The description of a notebook cell response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookCellResponse {
    /// The attributes of a notebook cell response. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
    /// `log_stream`. [More information on each graph visualization type.](<https://docs.datadoghq.com/dashboards/widgets/>)
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV1::model::NotebookCellResponseAttributes,
    /// Notebook cell ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Type of the Notebook Cell resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::NotebookCellResourceType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookCellResponse {
    pub fn new(
        attributes: crate::datadogV1::model::NotebookCellResponseAttributes,
        id: String,
        type_: crate::datadogV1::model::NotebookCellResourceType,
    ) -> NotebookCellResponse {
        NotebookCellResponse {
            attributes,
            id,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookCellResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookCellResponseVisitor;
        impl<'a> Visitor<'a> for NotebookCellResponseVisitor {
            type Value = NotebookCellResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::NotebookCellResponseAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::NotebookCellResourceType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _attributes) = attributes {
                                match _attributes {
                                    crate::datadogV1::model::NotebookCellResponseAttributes::UnparsedObject(_attributes) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::NotebookCellResourceType::UnparsedObject(_type_) => {
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

                let content = NotebookCellResponse {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookCellResponseVisitor)
    }
}
