// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the batch rows query response data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BatchRowsQueryResponseDataRelationships {
    /// Relationship data containing the list of matching rows.
    #[serde(rename = "rows")]
    pub rows: Option<crate::datadogV2::model::BatchRowsQueryResponseDataRelationshipsRows>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BatchRowsQueryResponseDataRelationships {
    pub fn new() -> BatchRowsQueryResponseDataRelationships {
        BatchRowsQueryResponseDataRelationships {
            rows: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn rows(
        mut self,
        value: crate::datadogV2::model::BatchRowsQueryResponseDataRelationshipsRows,
    ) -> Self {
        self.rows = Some(value);
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

impl Default for BatchRowsQueryResponseDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BatchRowsQueryResponseDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BatchRowsQueryResponseDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for BatchRowsQueryResponseDataRelationshipsVisitor {
            type Value = BatchRowsQueryResponseDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rows: Option<
                    crate::datadogV2::model::BatchRowsQueryResponseDataRelationshipsRows,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rows" => {
                            if v.is_null() {
                                continue;
                            }
                            rows = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BatchRowsQueryResponseDataRelationships {
                    rows,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BatchRowsQueryResponseDataRelationshipsVisitor)
    }
}
