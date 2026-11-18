// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Members of the previewed segment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentPreviewResponseAttributes {
    /// Rows of the preview, each as a map of column names to values.
    #[serde(rename = "hits")]
    pub hits: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Total number of entities in the segment.
    #[serde(rename = "total_rows")]
    pub total_rows: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentPreviewResponseAttributes {
    pub fn new(
        hits: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
        total_rows: i64,
    ) -> SegmentPreviewResponseAttributes {
        SegmentPreviewResponseAttributes {
            hits,
            total_rows,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SegmentPreviewResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentPreviewResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentPreviewResponseAttributesVisitor {
            type Value = SegmentPreviewResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hits: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>> =
                    None;
                let mut total_rows: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hits" => {
                            hits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_rows" => {
                            total_rows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let hits = hits.ok_or_else(|| M::Error::missing_field("hits"))?;
                let total_rows = total_rows.ok_or_else(|| M::Error::missing_field("total_rows"))?;

                let content = SegmentPreviewResponseAttributes {
                    hits,
                    total_rows,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentPreviewResponseAttributesVisitor)
    }
}
