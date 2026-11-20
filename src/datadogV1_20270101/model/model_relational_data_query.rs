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
pub struct RelationalDataQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::RelationalDataQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "filter")]
    pub filter: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::RelationalDataQuerySort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl RelationalDataQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::RelationalDataQueryDataSource, table_name: String) -> RelationalDataQuery {
        RelationalDataQuery {
            data_source,
            filter: None,
            limit: None,
            sort: None,
            table_name,
            _unparsed: false,
        }
    }
    
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::RelationalDataQuerySort) -> Self {
        self.sort = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for RelationalDataQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RelationalDataQueryVisitor;
        impl<'a> Visitor<'a> for RelationalDataQueryVisitor {
            type Value = RelationalDataQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::RelationalDataQueryDataSource> = None;
                let mut filter: Option<String> = None;
                let mut limit: Option<f64> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::RelationalDataQuerySort> = None;
                let mut table_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::RelationalDataQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = RelationalDataQuery {
                    data_source,
                    filter,
                    limit,
                    sort,
                    table_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RelationalDataQueryVisitor)
    }
}
