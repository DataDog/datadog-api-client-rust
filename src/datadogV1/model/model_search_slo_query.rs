// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
/// to be used because this will sum up all request counts instead of averaging them, or taking the max or
/// min of all of those requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchSLOQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator")]
    pub denominator: Option<String>,
    /// Metric names used in the query's numerator and denominator.
    /// This field will return null and will be implemented in the next version of this endpoint.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator")]
    pub numerator: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchSLOQuery {
    pub fn new() -> SearchSLOQuery {
        SearchSLOQuery {
            denominator: None,
            metrics: None,
            numerator: None,
            _unparsed: false,
        }
    }

    pub fn denominator(mut self, value: String) -> Self {
        self.denominator = Some(value);
        self
    }

    pub fn metrics(mut self, value: Option<Vec<String>>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn numerator(mut self, value: String) -> Self {
        self.numerator = Some(value);
        self
    }
}

impl Default for SearchSLOQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchSLOQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchSLOQueryVisitor;
        impl<'a> Visitor<'a> for SearchSLOQueryVisitor {
            type Value = SearchSLOQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denominator: Option<String> = None;
                let mut metrics: Option<Option<Vec<String>>> = None;
                let mut numerator: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denominator" => {
                            if v.is_null() {
                                continue;
                            }
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            if v.is_null() {
                                continue;
                            }
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchSLOQuery {
                    denominator,
                    metrics,
                    numerator,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchSLOQueryVisitor)
    }
}
