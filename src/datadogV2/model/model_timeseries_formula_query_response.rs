// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A message containing one response to a timeseries query made with timeseries formula query request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesFormulaQueryResponse {
    /// A message containing the response to a timeseries query.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::TimeseriesResponse>>,
    /// The error generated by the request.
    #[serde(rename = "errors")]
    pub errors: Option<String>,
}

impl TimeseriesFormulaQueryResponse {
    pub fn new() -> TimeseriesFormulaQueryResponse {
        TimeseriesFormulaQueryResponse {
            data: None,
            errors: None,
        }
    }
}
