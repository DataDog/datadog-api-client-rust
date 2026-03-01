// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A reference table query block within a segment data query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentReferenceTable {
    /// The columns to include from the reference table.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV2::model::RumSegmentReferenceTableColumn>,
    /// An optional filter query for the reference table data.
    #[serde(rename = "filter_query")]
    pub filter_query: Option<String>,
    /// The join condition for a reference table query block.
    #[serde(rename = "join_condition")]
    pub join_condition: crate::datadogV2::model::RumSegmentReferenceTableJoinCondition,
    /// The name of this query block.
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the reference table.
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentReferenceTable {
    pub fn new(
        columns: Vec<crate::datadogV2::model::RumSegmentReferenceTableColumn>,
        join_condition: crate::datadogV2::model::RumSegmentReferenceTableJoinCondition,
        name: String,
        table_name: String,
    ) -> RumSegmentReferenceTable {
        RumSegmentReferenceTable {
            columns,
            filter_query: None,
            join_condition,
            name,
            table_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
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

impl<'de> Deserialize<'de> for RumSegmentReferenceTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentReferenceTableVisitor;
        impl<'a> Visitor<'a> for RumSegmentReferenceTableVisitor {
            type Value = RumSegmentReferenceTable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<
                    Vec<crate::datadogV2::model::RumSegmentReferenceTableColumn>,
                > = None;
                let mut filter_query: Option<String> = None;
                let mut join_condition: Option<
                    crate::datadogV2::model::RumSegmentReferenceTableJoinCondition,
                > = None;
                let mut name: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_query" => {
                            if v.is_null() {
                                continue;
                            }
                            filter_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_condition" => {
                            join_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let join_condition =
                    join_condition.ok_or_else(|| M::Error::missing_field("join_condition"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = RumSegmentReferenceTable {
                    columns,
                    filter_query,
                    join_condition,
                    name,
                    table_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentReferenceTableVisitor)
    }
}
