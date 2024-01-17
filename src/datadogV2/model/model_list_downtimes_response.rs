// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving all downtimes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListDowntimesResponse {
    /// An array of downtimes.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::DowntimeResponseData>>,
    /// Array of objects related to the downtimes.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::DowntimeResponseIncludedItem>>,
    /// Pagination metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::DowntimeMeta>>,
}

impl ListDowntimesResponse {
    pub fn new() -> ListDowntimesResponse {
        ListDowntimesResponse {
            data: None,
            included: None,
            meta: None,
        }
    }
}
