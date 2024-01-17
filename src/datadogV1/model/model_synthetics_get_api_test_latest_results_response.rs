// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with the latest Synthetic API test run.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGetAPITestLatestResultsResponse {
    /// Timestamp of the latest API test run.
    #[serde(rename = "last_timestamp_fetched")]
    pub last_timestamp_fetched: Option<i64>,
    /// Result of the latest API test run.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsAPITestResultShort>>,
}

impl SyntheticsGetAPITestLatestResultsResponse {
    pub fn new() -> SyntheticsGetAPITestLatestResultsResponse {
        SyntheticsGetAPITestLatestResultsResponse {
            last_timestamp_fetched: None,
            results: None,
        }
    }
}
