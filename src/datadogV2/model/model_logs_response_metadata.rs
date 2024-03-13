// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata associated with a request
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsResponseMetadata {
    /// The time elapsed in milliseconds
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::LogsResponseMetadataPage>,
    /// The identifier of the request
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::LogsAggregateResponseStatus>,
    /// A list of warnings (non fatal errors) encountered, partial results might be returned if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::LogsWarning>>,
}

impl LogsResponseMetadata {
    pub fn new() -> LogsResponseMetadata {
        LogsResponseMetadata {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            warnings: None,
        }
    }

    pub fn elapsed(mut self, value: i64) -> Self {
        self.elapsed = Some(value);
        self
    }

    pub fn page(mut self, value: crate::datadogV2::model::LogsResponseMetadataPage) -> Self {
        self.page = Some(value);
        self
    }

    pub fn request_id(mut self, value: String) -> Self {
        self.request_id = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::LogsAggregateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<crate::datadogV2::model::LogsWarning>) -> Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for LogsResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}
