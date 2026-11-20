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
pub struct TableRowGroupBy {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "format")]
    pub format: Option<crate::datadogV1_20270101::model::TableRowGroupByFormat>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_keys")]
    pub group_keys: Vec<crate::datadogV1_20270101::model::TableGroupKey>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TableRowGroupBy {
    pub fn new(group_keys: Vec<crate::datadogV1_20270101::model::TableGroupKey>, name: String) -> TableRowGroupBy {
        TableRowGroupBy {
            alias: None,
            format: None,
            group_keys,
            is_hidden: None,
            name,
            _unparsed: false,
        }
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
    pub fn format(mut self, value: crate::datadogV1_20270101::model::TableRowGroupByFormat) -> Self {
        self.format = Some(value);
        self
    }
    
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TableRowGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TableRowGroupByVisitor;
        impl<'a> Visitor<'a> for TableRowGroupByVisitor {
            type Value = TableRowGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alias: Option<String> = None;
                let mut format: Option<crate::datadogV1_20270101::model::TableRowGroupByFormat> = None;
                let mut group_keys: Option<Vec<crate::datadogV1_20270101::model::TableGroupKey>> = None;
                let mut is_hidden: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "format" => {
                            if v.is_null() {
                                continue;
                            }
                            format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_keys" => {
                            group_keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "is_hidden" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let group_keys = group_keys.ok_or_else(|| M::Error::missing_field("group_keys"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = TableRowGroupBy {
                    alias,
                    format,
                    group_keys,
                    is_hidden,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TableRowGroupByVisitor)
    }
}
