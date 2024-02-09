// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving all downtime matches for a monitor.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponse {
    /// An array of downtime matches.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::MonitorDowntimeMatchResponseData>>,
    /// Pagination metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::DowntimeMeta>,
}

impl MonitorDowntimeMatchResponse {
    pub fn new() -> MonitorDowntimeMatchResponse {
        MonitorDowntimeMatchResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::MonitorDowntimeMatchResponseData>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::DowntimeMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for MonitorDowntimeMatchResponse {
    fn default() -> Self {
        Self::new()
    }
}
