// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a journey funnel request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyFunnelQuery {
    /// Defines the metric computed at each funnel step.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV2::model::ProductAnalyticsJourneyFunnelCompute>,
    /// Segments the funnel by the values of one or more facets.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>>,
    /// Defines the steps of the journey and the filters applied to it.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyFunnelQuery {
    pub fn new(
        search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    ) -> ProductAnalyticsJourneyFunnelQuery {
        ProductAnalyticsJourneyFunnelQuery {
            compute: None,
            group_by: None,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compute(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneyFunnelCompute,
    ) -> Self {
        self.compute = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyFunnelQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyFunnelQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyFunnelQueryVisitor {
            type Value = ProductAnalyticsJourneyFunnelQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneyFunnelCompute,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsGraphQueryGroupBy>,
                > = None;
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
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsJourneyFunnelQuery {
                    compute,
                    group_by,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyFunnelQueryVisitor)
    }
}
