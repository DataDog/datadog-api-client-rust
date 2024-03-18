// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object containing all the query parameters.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansListRequestAttributes {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SpansQueryFilter>,
    /// Global query options that are used during the query.
    /// Note: You should only supply timezone or time offset but not both otherwise the query will fail.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SpansQueryOptions>,
    /// Paging attributes for listing spans.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SpansListRequestPage>,
    /// Sort parameters when querying spans.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::SpansSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansListRequestAttributes {
    pub fn new() -> SpansListRequestAttributes {
        SpansListRequestAttributes {
            filter: None,
            options: None,
            page: None,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::SpansQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::SpansQueryOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::SpansListRequestPage) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::SpansSort) -> Self {
        self.sort = Some(value);
        self
    }
}

impl Default for SpansListRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansListRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansListRequestAttributesVisitor;
        impl<'a> Visitor<'a> for SpansListRequestAttributesVisitor {
            type Value = SpansListRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::SpansQueryFilter> = None;
                let mut options: Option<crate::datadogV2::model::SpansQueryOptions> = None;
                let mut page: Option<crate::datadogV2::model::SpansListRequestPage> = None;
                let mut sort: Option<crate::datadogV2::model::SpansSort> = None;
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
                                    crate::datadogV2::model::SpansSort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SpansListRequestAttributes {
                    filter,
                    options,
                    page,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansListRequestAttributesVisitor)
    }
}
