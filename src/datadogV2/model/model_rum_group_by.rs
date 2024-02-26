// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A group-by rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn histogram(&mut self, value: crate::datadogV2::model::RUMGroupByHistogram) -> &mut Self {
        self.histogram = Some(value);
        self
    }

    pub fn limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn missing(&mut self, value: crate::datadogV2::model::RUMGroupByMissing) -> &mut Self {
        self.missing = Some(value);
        self
    }

    pub fn sort(&mut self, value: crate::datadogV2::model::RUMAggregateSort) -> &mut Self {
        self.sort = Some(value);
        self
    }

    pub fn total(&mut self, value: crate::datadogV2::model::RUMGroupByTotal) -> &mut Self {
        self.total = Some(value);
        self
    }
}
