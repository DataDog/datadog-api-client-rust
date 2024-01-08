// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response of a monitor group search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorGroupSearchResponse {
    /// The counts of monitor groups per different criteria.
    #[serde(rename = "counts")]
    pub counts: Option<Box<crate::datadogV1::model::MonitorGroupSearchResponseCounts>>,
    /// The list of found monitor groups.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV1::model::MonitorGroupSearchResult>>,
    /// Metadata about the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<Box<crate::datadogV1::model::MonitorSearchResponseMetadata>>,
}

impl MonitorGroupSearchResponse {
    pub fn new() -> MonitorGroupSearchResponse {
        MonitorGroupSearchResponse {
            counts: None,
            groups: None,
            metadata: None,
        }
    }
}
impl Default for MonitorGroupSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}
