// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A group by rule
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGroupBy {
    /// The name of the facet to use (required)
    #[serde(rename = "facet")]
    pub facet: String,
    /// Used to perform a histogram computation (only for measure facets).
    /// Note: at most 100 buckets are allowed, the number of buckets is (max - min)/interval.
    #[serde(rename = "histogram")]
    pub histogram: Option<Box<crate::datadogV2::model::LogsGroupByHistogram>>,
    /// The maximum buckets to return for this group by. Note: at most 10000 buckets are allowed.
    /// If grouping by multiple facets, the product of limits must not exceed 10000.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// The value to use for logs that don't have the facet used to group by
    #[serde(rename = "missing")]
    pub missing: Option<Box<crate::datadogV2::model::LogsGroupByMissing>>,
    /// A sort rule
    #[serde(rename = "sort")]
    pub sort: Option<Box<crate::datadogV2::model::LogsAggregateSort>>,
    /// A resulting object to put the given computes in over all the matching records.
    #[serde(rename = "total")]
    pub total: Option<Box<crate::datadogV2::model::LogsGroupByTotal>>,
}

impl LogsGroupBy {
    pub fn new(facet: String) -> LogsGroupBy {
        LogsGroupBy {
            facet,
            histogram: None,
            limit: None,
            missing: None,
            sort: None,
            total: None,
        }
    }
}