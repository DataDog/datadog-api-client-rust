// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object for the logs aggregate API endpoint
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateResponse {
    /// The query results
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::LogsAggregateResponseData>,
    /// The metadata associated with a request
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::LogsResponseMetadata>,
}

impl LogsAggregateResponse {
    pub fn new() -> LogsAggregateResponse {
        LogsAggregateResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::LogsAggregateResponseData) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::LogsResponseMetadata) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for LogsAggregateResponse {
    fn default() -> Self {
        Self::new()
    }
}
