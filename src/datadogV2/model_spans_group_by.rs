// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansGroupBy {
    /// The name of the facet to use (required).
    #[serde(rename = "facet", skip_serializing_if = "Option::is_none")]
    pub facet: String,
    /// Used to perform a histogram computation (only for measure facets).
Note: At most 100 buckets are allowed, the number of buckets is (max - min)/interval.
    #[serde(rename = "histogram")]
    pub histogram: SpansGroupByHistogram,
    /// The maximum buckets to return for this group by.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// The value to use for spans that don't have the facet used to group by.
    #[serde(rename = "missing", skip_serializing_if = "Option::is_none")]
    pub missing: SpansGroupByMissing,
    /// A sort rule.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: SpansAggregateSort,
    /// A resulting object to put the given computes in over all the matching records.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: SpansGroupByTotal,
}

