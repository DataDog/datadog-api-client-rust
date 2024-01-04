// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response form a monitor search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponse {
    /// The counts of monitors per different criteria.
    #[serde(rename = "counts")]
    pub counts: Option<Box<crate::datadogV1::model::MonitorSearchResponseCounts>>,
    /// Metadata about the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::MonitorSearchResponseMetadata>>,
    /// The list of found monitors.
    #[serde(rename = "monitors")]
    pub monitors: Option<Vec<crate::datadogV1::model::MonitorSearchResult>>,
}

impl MonitorSearchResponse {
    pub fn new() -> MonitorSearchResponse {
        MonitorSearchResponse {
            counts: None,
            metadata: None,
            monitors: None,
        }
    }
}
