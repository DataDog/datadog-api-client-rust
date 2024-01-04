// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing all the query parameters.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateRequestAttributes {
    /// The list of metrics or timeseries to compute for the retrieved buckets.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV2::model::SpansCompute>>,
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::SpansQueryFilter>>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::SpansGroupBy>>,
    /// Global query options that are used during the query.
    /// Note: You should only supply timezone or time offset but not both otherwise the query will fail.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::SpansQueryOptions>>,
}

impl SpansAggregateRequestAttributes {
    pub fn new() -> SpansAggregateRequestAttributes {
        SpansAggregateRequestAttributes {
            compute: None,
            filter: None,
            group_by: None,
            options: None,
        }
    }
}