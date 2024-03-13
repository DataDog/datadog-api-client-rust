// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object sent with the request to retrieve a list of events from your organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsListRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::EventsQueryFilter>,
    /// The global query options that are used. Either provide a timezone or a time offset but not both,
    /// otherwise the query fails.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::EventsQueryOptions>,
    /// Pagination settings.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::EventsRequestPage>,
    /// The sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::EventsSort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsListRequest {
    pub fn new() -> EventsListRequest {
        EventsListRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::EventsQueryFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::EventsQueryOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::EventsRequestPage) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::EventsSort) -> Self {
        self.sort = Some(value);
        self
    }
}

impl Default for EventsListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventsListRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsListRequestVisitor;
        impl<'a> Visitor<'a> for EventsListRequestVisitor {
            type Value = EventsListRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV2::model::EventsQueryFilter> = None;
                let mut options: Option<crate::datadogV2::model::EventsQueryOptions> = None;
                let mut page: Option<crate::datadogV2::model::EventsRequestPage> = None;
                let mut sort: Option<crate::datadogV2::model::EventsSort> = None;
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
                                    crate::datadogV2::model::EventsSort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = EventsListRequest {
                    filter,
                    options,
                    page,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsListRequestVisitor)
    }
}
