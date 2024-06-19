// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A dimension on which to split a query's results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsGroupBy {
    /// The facet by which to split groups.
    #[serde(rename = "facet")]
    pub facet: String,
    /// The maximum buckets to return for this group by. Note: at most 10000 buckets are allowed.
    /// If grouping by multiple facets, the product of limits must not exceed 10000.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The dimension by which to sort a query's results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::EventsGroupBySort>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsGroupBy {
    pub fn new(facet: String) -> EventsGroupBy {
        EventsGroupBy {
            facet,
            limit: None,
            sort: None,
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::EventsGroupBySort) -> Self {
        self.sort = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for EventsGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsGroupByVisitor;
        impl<'a> Visitor<'a> for EventsGroupByVisitor {
            type Value = EventsGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut limit: Option<i32> = None;
                let mut sort: Option<crate::datadogV2::model::EventsGroupBySort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = EventsGroupBy {
                    facet,
                    limit,
                    sort,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsGroupByVisitor)
    }
}
