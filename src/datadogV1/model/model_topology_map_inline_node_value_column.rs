// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Column of numeric node values. Nodes display non-null values in order of column `id`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyMapInlineNodeValueColumn {
    /// Position of the value in the node, as `values[N]`. The smallest N appears first.
    #[serde(rename = "id")]
    pub id: String,
    /// Column of numbers.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV1::model::TopologyMapInlineColumnKindScalar,
    /// Display options for a value column.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::TopologyMapInlineNodeValueColumnMeta>,
    /// Value for each node. Null values are not displayed.
    #[serde(rename = "values")]
    pub values: Vec<Option<f64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyMapInlineNodeValueColumn {
    pub fn new(
        id: String,
        kind: crate::datadogV1::model::TopologyMapInlineColumnKindScalar,
        values: Vec<Option<f64>>,
    ) -> TopologyMapInlineNodeValueColumn {
        TopologyMapInlineNodeValueColumn {
            id,
            kind,
            meta: None,
            values,
            _unparsed: false,
        }
    }

    pub fn meta(
        mut self,
        value: crate::datadogV1::model::TopologyMapInlineNodeValueColumnMeta,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TopologyMapInlineNodeValueColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyMapInlineNodeValueColumnVisitor;
        impl<'a> Visitor<'a> for TopologyMapInlineNodeValueColumnVisitor {
            type Value = TopologyMapInlineNodeValueColumn;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut kind: Option<crate::datadogV1::model::TopologyMapInlineColumnKindScalar> =
                    None;
                let mut meta: Option<
                    crate::datadogV1::model::TopologyMapInlineNodeValueColumnMeta,
                > = None;
                let mut values: Option<Vec<Option<f64>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV1::model::TopologyMapInlineColumnKindScalar::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = TopologyMapInlineNodeValueColumn {
                    id,
                    kind,
                    meta,
                    values,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyMapInlineNodeValueColumnVisitor)
    }
}
