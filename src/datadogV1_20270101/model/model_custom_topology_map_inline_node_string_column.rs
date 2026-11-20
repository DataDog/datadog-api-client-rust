// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomTopologyMapInlineNodeStringColumn {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "id")]
    pub id: crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnId,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnKind,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "values")]
    pub values: Vec<Option<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CustomTopologyMapInlineNodeStringColumn {
    pub fn new(id: crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnId, kind: crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnKind, values: Vec<Option<String>>) -> CustomTopologyMapInlineNodeStringColumn {
        CustomTopologyMapInlineNodeStringColumn {
            id,
            kind,
            values,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CustomTopologyMapInlineNodeStringColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomTopologyMapInlineNodeStringColumnVisitor;
        impl<'a> Visitor<'a> for CustomTopologyMapInlineNodeStringColumnVisitor {
            type Value = CustomTopologyMapInlineNodeStringColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnId> = None;
                let mut kind: Option<crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnKind> = None;
                let mut values: Option<Vec<Option<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _id) = id {
                                match _id {
                                    crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnId::UnparsedObject(_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV1_20270101::model::CustomTopologyMapInlineNodeStringColumnKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = CustomTopologyMapInlineNodeStringColumn {
                    id,
                    kind,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomTopologyMapInlineNodeStringColumnVisitor)
    }
}
