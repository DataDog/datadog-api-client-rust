// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Message for specifying limits to the number of values returned by a query.
/// This limit is only for scalar queries and has no effect on timeseries queries.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaLimit {
    /// The number of results to which to limit.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::QuerySortOrder>,
}

impl FormulaLimit {
    pub fn new() -> FormulaLimit {
        FormulaLimit {
            count: None,
            order: None,
        }
    }
}