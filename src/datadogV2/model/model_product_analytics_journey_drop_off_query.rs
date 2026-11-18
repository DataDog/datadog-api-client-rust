// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for a journey drop-off analysis request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyDropOffQuery {
    /// Sampling configuration for a query.
    #[serde(rename = "sampling")]
    pub sampling: Option<crate::datadogV2::model::ProductAnalyticsSampling>,
    /// Defines the steps of the journey and the filters applied to it.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    /// A reference to a single step of the journey.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::ProductAnalyticsJourneyNodeTarget>,
    /// Maximum number of issues to return.
    #[serde(rename = "top_issue_count")]
    pub top_issue_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyDropOffQuery {
    pub fn new(
        search: crate::datadogV2::model::ProductAnalyticsJourneySearch,
    ) -> ProductAnalyticsJourneyDropOffQuery {
        ProductAnalyticsJourneyDropOffQuery {
            sampling: None,
            search,
            target: None,
            top_issue_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn sampling(mut self, value: crate::datadogV2::model::ProductAnalyticsSampling) -> Self {
        self.sampling = Some(value);
        self
    }

    pub fn target(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneyNodeTarget,
    ) -> Self {
        self.target = Some(value);
        self
    }

    pub fn top_issue_count(mut self, value: i64) -> Self {
        self.top_issue_count = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyDropOffQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyDropOffQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyDropOffQueryVisitor {
            type Value = ProductAnalyticsJourneyDropOffQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut sampling: Option<crate::datadogV2::model::ProductAnalyticsSampling> = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsJourneySearch> =
                    None;
                let mut target: Option<crate::datadogV2::model::ProductAnalyticsJourneyNodeTarget> =
                    None;
                let mut top_issue_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "sampling" => {
                            if v.is_null() {
                                continue;
                            }
                            sampling = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "top_issue_count" => {
                            if v.is_null() {
                                continue;
                            }
                            top_issue_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsJourneyDropOffQuery {
                    sampling,
                    search,
                    target,
                    top_issue_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyDropOffQueryVisitor)
    }
}
