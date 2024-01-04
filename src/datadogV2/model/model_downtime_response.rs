// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Downtiming gives you greater control over monitor notifications by
/// allowing you to globally exclude scopes from alerting.
/// Downtime settings, which can be scheduled with start and end times,
/// prevent all alerting related to specified Datadog tags.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeResponse {
    /// Downtime data.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::DowntimeResponseData>>,
    /// Array of objects related to the downtime that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::DowntimeResponseIncludedItem>>,
}

impl DowntimeResponse {
    pub fn new() -> DowntimeResponse {
        DowntimeResponse {
            data: None,
            included: None,
        }
    }
}
