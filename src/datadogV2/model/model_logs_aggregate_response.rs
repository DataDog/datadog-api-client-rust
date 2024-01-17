// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object for the logs aggregate API endpoint
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateResponse {
    /// The query results
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::LogsAggregateResponseData>>,
    /// The metadata associated with a request
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::LogsResponseMetadata>>,
}

impl LogsAggregateResponse {
    pub fn new() -> LogsAggregateResponse {
        LogsAggregateResponse {
            data: None,
            meta: None,
        }
    }
}
