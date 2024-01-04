// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata associated with a request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::SpansResponseMetadataPage>>,
    /// The identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SpansAggregateResponseStatus>,
    /// A list of warnings (non fatal errors) encountered, partial results might be returned if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::SpansWarning>>,
}

impl SpansListResponseMetadata {
    pub fn new() -> SpansListResponseMetadata {
        SpansListResponseMetadata {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            warnings: None,
        }
    }
}