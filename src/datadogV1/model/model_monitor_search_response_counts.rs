// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The counts of monitors per different criteria.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResponseCounts {
    /// Search facets.
    #[serde(rename = "muted")]
    pub muted: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "status")]
    pub status: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "tag")]
    pub tag: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "type")]
    pub type_: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
}

impl MonitorSearchResponseCounts {
    pub fn new() -> MonitorSearchResponseCounts {
        MonitorSearchResponseCounts {
            muted: None,
            status: None,
            tag: None,
            type_: None,
        }
    }
}
impl Default for MonitorSearchResponseCounts {
    fn default() -> Self {
        Self::new()
    }
}
