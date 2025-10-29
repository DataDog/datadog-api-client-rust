// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentDataAttributesDataQueryReferenceTableItems {
    #[serde(rename = "columns")]
    pub columns: Option<
        Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsColumnsItems>,
    >,
    #[serde(rename = "filter_query")]
    pub filter_query: Option<String>,
    #[serde(rename = "join_condition")]
    pub join_condition:
        crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsJoinCondition,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentDataAttributesDataQueryReferenceTableItems {
    pub fn new(
        join_condition: crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsJoinCondition,
        table_name: String,
    ) -> SegmentDataAttributesDataQueryReferenceTableItems {
        SegmentDataAttributesDataQueryReferenceTableItems {
            columns: None,
            filter_query: None,
            join_condition,
            name: None,
            table_name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn columns(
        mut self,
        value: Vec<
            crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsColumnsItems,
        >,
    ) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl<'de> Deserialize<'de> for SegmentDataAttributesDataQueryReferenceTableItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentDataAttributesDataQueryReferenceTableItemsVisitor;
        impl<'a> Visitor<'a> for SegmentDataAttributesDataQueryReferenceTableItemsVisitor {
            type Value = SegmentDataAttributesDataQueryReferenceTableItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsColumnsItems>> = None;
                let mut filter_query: Option<String> = None;
                let mut join_condition: Option<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItemsJoinCondition> = None;
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
                            if v.is_null() {
                                continue;
                            }
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
                            if v.is_null() {
                                continue;
                            }
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
                let join_condition =
                    join_condition.ok_or_else(|| M::Error::missing_field("join_condition"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = SegmentDataAttributesDataQueryReferenceTableItems {
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

        deserializer.deserialize_any(SegmentDataAttributesDataQueryReferenceTableItemsVisitor)
    }
}
