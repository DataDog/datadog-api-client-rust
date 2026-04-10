// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User journey funnel query definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFunnelQuery {
    /// Compute configuration for user journey funnel.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1::model::ProductAnalyticsFunnelCompute>,
    /// Data source for user journey funnel queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
    /// Group by configuration.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::ProductAnalyticsFunnelGroupBy>>,
    /// User journey search configuration.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::UserJourneySearch,
    /// Subquery ID.
    #[serde(rename = "subquery_id")]
    pub subquery_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFunnelQuery {
    pub fn new(
        data_source: crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
        search: crate::datadogV1::model::UserJourneySearch,
    ) -> ProductAnalyticsFunnelQuery {
        ProductAnalyticsFunnelQuery {
            compute: None,
            data_source,
            group_by: None,
            search,
            subquery_id: None,
            _unparsed: false,
        }
    }

    pub fn compute(
        mut self,
        value: crate::datadogV1::model::ProductAnalyticsFunnelCompute,
    ) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::ProductAnalyticsFunnelGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn subquery_id(mut self, value: String) -> Self {
        self.subquery_id = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsFunnelQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFunnelQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFunnelQueryVisitor {
            type Value = ProductAnalyticsFunnelQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::ProductAnalyticsFunnelCompute> =
                    None;
                let mut data_source: Option<
                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::ProductAnalyticsFunnelGroupBy>,
                > = None;
                let mut search: Option<crate::datadogV1::model::UserJourneySearch> = None;
                let mut subquery_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::ProductAnalyticsFunnelDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subquery_id" => {
                            if v.is_null() {
                                continue;
                            }
                            subquery_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsFunnelQuery {
                    compute,
                    data_source,
                    group_by,
                    search,
                    subquery_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFunnelQueryVisitor)
    }
}
