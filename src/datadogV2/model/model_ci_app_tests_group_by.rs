// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A group-by rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestsGroupBy {
    /// The name of the facet to use (required).
    #[serde(rename = "facet")]
    pub facet: String,
    /// Used to perform a histogram computation (only for measure facets).
    /// At most, 100 buckets are allowed, the number of buckets is `(max - min)/interval`.
    #[serde(rename = "histogram")]
    pub histogram: Option<crate::datadogV2::model::CIAppGroupByHistogram>,
    /// The maximum buckets to return for this group-by.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The value to use for logs that don't have the facet used to group-by.
    #[serde(rename = "missing")]
    pub missing: Option<crate::datadogV2::model::CIAppGroupByMissing>,
    /// A sort rule.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::CIAppAggregateSort>,
    /// A resulting object to put the given computes in over all the matching records.
    #[serde(rename = "total")]
    pub total: Option<crate::datadogV2::model::CIAppGroupByTotal>,
}

impl CIAppTestsGroupBy {
    pub fn new(facet: String) -> CIAppTestsGroupBy {
        CIAppTestsGroupBy {
            facet,
            histogram: None,
            limit: None,
            missing: None,
            sort: None,
            total: None,
        }
    }

    pub fn with_histogram(
        &mut self,
        value: crate::datadogV2::model::CIAppGroupByHistogram,
    ) -> &mut Self {
        self.histogram = Some(value);
        self
    }

    pub fn with_limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn with_missing(
        &mut self,
        value: crate::datadogV2::model::CIAppGroupByMissing,
    ) -> &mut Self {
        self.missing = Some(value);
        self
    }

    pub fn with_sort(&mut self, value: crate::datadogV2::model::CIAppAggregateSort) -> &mut Self {
        self.sort = Some(value);
        self
    }

    pub fn with_total(&mut self, value: crate::datadogV2::model::CIAppGroupByTotal) -> &mut Self {
        self.total = Some(value);
        self
    }
}
