// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updated SLO List widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOListWidgetQuery {
    /// Maximum number of results to display in the table.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Options for sorting results.
    #[serde(rename = "sort")]
    pub sort: Option<Vec<crate::datadogV1::model::WidgetFieldSort>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOListWidgetQuery {
    pub fn new(query_string: String) -> SLOListWidgetQuery {
        SLOListWidgetQuery {
            limit: None,
            query_string,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<crate::datadogV1::model::WidgetFieldSort>) -> Self {
        self.sort = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SLOListWidgetQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOListWidgetQueryVisitor;
        impl<'a> Visitor<'a> for SLOListWidgetQueryVisitor {
            type Value = SLOListWidgetQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut limit: Option<i64> = None;
                let mut query_string: Option<String> = None;
                let mut sort: Option<Vec<crate::datadogV1::model::WidgetFieldSort>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;

                let content = SLOListWidgetQuery {
                    limit,
                    query_string,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOListWidgetQueryVisitor)
    }
}
