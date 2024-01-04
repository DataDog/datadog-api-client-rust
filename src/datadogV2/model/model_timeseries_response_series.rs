// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

///
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesResponseSeries {
    /// List of tags that apply to a single response value.
    #[serde(rename = "group_tags")]
    pub group_tags: Option<Vec<String>>,
    /// The index of the query in the "formulas" array (or "queries" array if no "formulas" was specified).
    #[serde(rename = "query_index")]
    pub query_index: Option<i32>,
    /// Detailed information about the unit.
    /// The first element describes the "primary unit" (for example, `bytes` in `bytes per second`).
    /// The second element describes the "per unit" (for example, `second` in `bytes per second`).
    /// If the second element is not present, the API returns null.
    #[serde(rename = "unit")]
    pub unit: Option<Vec<crate::datadogV2::model::Unit>>,
}

impl TimeseriesResponseSeries {
    pub fn new() -> TimeseriesResponseSeries {
        TimeseriesResponseSeries {
            group_tags: None,
            query_index: None,
            unit: None,
        }
    }
}
