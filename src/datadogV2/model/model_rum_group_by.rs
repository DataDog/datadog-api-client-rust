// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A group-by rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMGroupBy {
    /// The name of the facet to use (required).
    #[serde(rename = "facet")]
    pub facet: String,
    /// Used to perform a histogram computation (only for measure facets).
    /// Note: At most 100 buckets are allowed, the number of buckets is (max - min)/interval.
    #[serde(rename = "histogram")]
    pub histogram: Option<crate::datadogV2::model::RUMGroupByHistogram>,
    /// The maximum buckets to return for this group-by.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The value to use for logs that don't have the facet used to group by.
    #[serde(rename = "missing")]
    pub missing: Option<crate::datadogV2::model::RUMGroupByMissing>,
    /// A sort rule.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::RUMAggregateSort>,
    /// A resulting object to put the given computes in over all the matching records.
    #[serde(rename = "total")]
    pub total: Option<crate::datadogV2::model::RUMGroupByTotal>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMGroupBy {
    pub fn new(facet: String) -> RUMGroupBy {
        RUMGroupBy {
            facet,
            histogram: None,
            limit: None,
            missing: None,
            sort: None,
            total: None,
            _unparsed: false,
        }
    }

    pub fn histogram(mut self, value: crate::datadogV2::model::RUMGroupByHistogram) -> Self {
        self.histogram = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn missing(mut self, value: crate::datadogV2::model::RUMGroupByMissing) -> Self {
        self.missing = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::RUMAggregateSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn total(mut self, value: crate::datadogV2::model::RUMGroupByTotal) -> Self {
        self.total = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for RUMGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMGroupByVisitor;
        impl<'a> Visitor<'a> for RUMGroupByVisitor {
            type Value = RUMGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut histogram: Option<crate::datadogV2::model::RUMGroupByHistogram> = None;
                let mut limit: Option<i64> = None;
                let mut missing: Option<crate::datadogV2::model::RUMGroupByMissing> = None;
                let mut sort: Option<crate::datadogV2::model::RUMAggregateSort> = None;
                let mut total: Option<crate::datadogV2::model::RUMGroupByTotal> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "histogram" => {
                            if v.is_null() {
                                continue;
                            }
                            histogram = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "missing" => {
                            if v.is_null() {
                                continue;
                            }
                            missing = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _missing) = missing {
                                match _missing {
                                    crate::datadogV2::model::RUMGroupByMissing::UnparsedObject(
                                        _missing,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _total) = total {
                                match _total {
                                    crate::datadogV2::model::RUMGroupByTotal::UnparsedObject(
                                        _total,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;

                let content = RUMGroupBy {
                    facet,
                    histogram,
                    limit,
                    missing,
                    sort,
                    total,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMGroupByVisitor)
    }
}
