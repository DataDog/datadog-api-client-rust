// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a journey timeseries request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFormulaJourneyQuery {
    /// Defines the metric computed over the journey.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::ProductAnalyticsGraphQueryCompute,
    /// Segments the results by the values of one or more facets.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>>,
    /// Caller-defined identifier echoed back in the results.
    #[serde(rename = "query_id")]
    pub query_id: Option<String>,
    /// Defines the steps of the journey and the filters applied to it.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFormulaJourneyQuery {
    pub fn new(
        compute: crate::datadogV2::model::ProductAnalyticsGraphQueryCompute,
        search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    ) -> ProductAnalyticsFormulaJourneyQuery {
        ProductAnalyticsFormulaJourneyQuery {
            compute,
            group_by: None,
            query_id: None,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn query_id(mut self, value: String) -> Self {
        self.query_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsFormulaJourneyQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFormulaJourneyQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFormulaJourneyQueryVisitor {
            type Value = ProductAnalyticsFormulaJourneyQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<
                    crate::datadogV2::model::ProductAnalyticsGraphQueryCompute,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
                > = None;
                let mut query_id: Option<String> = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsJourneySearch> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_id" => {
                            if v.is_null() {
                                continue;
                            }
                            query_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsFormulaJourneyQuery {
                    compute,
                    group_by,
                    query_id,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFormulaJourneyQueryVisitor)
    }
}
