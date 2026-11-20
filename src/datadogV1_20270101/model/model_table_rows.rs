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
pub struct TableRows {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Vec<crate::datadogV1_20270101::model::TableRowGroupBy>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "join_type")]
    pub join_type: Option<crate::datadogV1_20270101::model::TableRowsJoinType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: crate::datadogV1_20270101::model::TableSort,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableRows {
    pub fn new(group_by: Vec<crate::datadogV1_20270101::model::TableRowGroupBy>, sort: crate::datadogV1_20270101::model::TableSort) -> TableRows {
        TableRows {
            group_by,
            join_type: None,
            sort,
            _unparsed: false,
        }
    }
    
    pub fn join_type(mut self, value: crate::datadogV1_20270101::model::TableRowsJoinType) -> Self {
        self.join_type = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TableRows {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableRowsVisitor;
        impl<'a> Visitor<'a> for TableRowsVisitor {
            type Value = TableRows;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::TableRowGroupBy>> = None;
                let mut join_type: Option<crate::datadogV1_20270101::model::TableRowsJoinType> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::TableSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "join_type" => {
                            if v.is_null() {
                                continue;
                            }
                            join_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _join_type) = join_type {
                                match _join_type {
                                    crate::datadogV1_20270101::model::TableRowsJoinType::UnparsedObject(_join_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "sort" => {
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let sort = sort.ok_or_else(|| M::Error::missing_field("sort"))?;

                let content = TableRows {
                    group_by,
                    join_type,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableRowsVisitor)
    }
}
