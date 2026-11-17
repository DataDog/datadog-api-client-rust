// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Segment definition to preview and the columns to return.
/// Without columns, the response returns the user ID, or the account ID for account segments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentPreviewAttributes {
    /// Account profile columns to return.
    #[serde(rename = "account_store_columns")]
    pub account_store_columns: Option<Vec<String>>,
    /// Defines who belongs to the segment. It contains one or more blocks, up to 20 in total.
    /// Use `combination` to combine named blocks with a boolean formula such as `A AND NOT B`.
    #[serde(rename = "data_query")]
    pub data_query: crate::datadogV2::model::SegmentDataQuery,
    /// Maximum number of rows to return, up to 1000. Defaults to 1000.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Reference table columns to return.
    #[serde(rename = "reference_table_columns")]
    pub reference_table_columns: Option<Vec<String>>,
    /// User profile columns to return.
    #[serde(rename = "user_store_columns")]
    pub user_store_columns: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentPreviewAttributes {
    pub fn new(data_query: crate::datadogV2::model::SegmentDataQuery) -> SegmentPreviewAttributes {
        SegmentPreviewAttributes {
            account_store_columns: None,
            data_query,
            limit: None,
            reference_table_columns: None,
            user_store_columns: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_store_columns(mut self, value: Vec<String>) -> Self {
        self.account_store_columns = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn reference_table_columns(mut self, value: Vec<String>) -> Self {
        self.reference_table_columns = Some(value);
        self
    }

    pub fn user_store_columns(mut self, value: Vec<String>) -> Self {
        self.user_store_columns = Some(value);
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

impl<'de> Deserialize<'de> for SegmentPreviewAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentPreviewAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentPreviewAttributesVisitor {
            type Value = SegmentPreviewAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_store_columns: Option<Vec<String>> = None;
                let mut data_query: Option<crate::datadogV2::model::SegmentDataQuery> = None;
                let mut limit: Option<i64> = None;
                let mut reference_table_columns: Option<Vec<String>> = None;
                let mut user_store_columns: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_store_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            account_store_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_query" => {
                            data_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_table_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_table_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_store_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            user_store_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_query = data_query.ok_or_else(|| M::Error::missing_field("data_query"))?;

                let content = SegmentPreviewAttributes {
                    account_store_columns,
                    data_query,
                    limit,
                    reference_table_columns,
                    user_store_columns,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentPreviewAttributesVisitor)
    }
}
