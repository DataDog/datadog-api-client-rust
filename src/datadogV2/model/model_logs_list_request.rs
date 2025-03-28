// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request for a logs list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsListRequest {
    /// The search and filter query settings
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::LogsQueryFilter>,
    /// Global query options that are used during the query.
    /// Note: These fields are currently deprecated and do not affect the query results.
    #[deprecated]
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::LogsQueryOptions>,
    /// Paging attributes for listing logs.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LogsListRequestPage>,
    /// Sort parameters when querying logs.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::LogsSort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsListRequest {
    pub fn new() -> LogsListRequest {
        #[allow(deprecated)]
        LogsListRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn filter(mut self, value: crate::datadogV2::model::LogsQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn options(mut self, value: crate::datadogV2::model::LogsQueryOptions) -> Self {
        self.options = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn page(mut self, value: crate::datadogV2::model::LogsListRequestPage) -> Self {
        self.page = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sort(mut self, value: crate::datadogV2::model::LogsSort) -> Self {
        self.sort = Some(value);
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

impl Default for LogsListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsListRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsListRequestVisitor;
        impl<'a> Visitor<'a> for LogsListRequestVisitor {
            type Value = LogsListRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::LogsQueryFilter> = None;
                let mut options: Option<crate::datadogV2::model::LogsQueryOptions> = None;
                let mut page: Option<crate::datadogV2::model::LogsListRequestPage> = None;
                let mut sort: Option<crate::datadogV2::model::LogsSort> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV2::model::LogsSort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = LogsListRequest {
                    filter,
                    options,
                    page,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsListRequestVisitor)
    }
}
