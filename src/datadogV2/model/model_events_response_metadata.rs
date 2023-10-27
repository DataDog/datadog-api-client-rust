// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventsResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// Pagination attributes.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::EventsResponseMetadataPage>>,
    /// The identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The request status.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// A list of warnings (non-fatal errors) encountered. Partial results might be returned if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::EventsWarning>>,
}

impl EventsResponseMetadata {
    /// The metadata associated with a request.
    pub fn new() -> EventsResponseMetadata {
        EventsResponseMetadata {
            elapsed: None,
            page: None,
            request_id: None,
            status: None,
            warnings: None,
        }
    }
}
