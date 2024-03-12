// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Information on your dashboard lists.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListListResponse {
    /// List of all your dashboard lists.
    #[serde(rename = "dashboard_lists")]
    pub dashboard_lists: Option<Vec<crate::datadogV1::model::DashboardList>>,
}

impl DashboardListListResponse {
    pub fn new() -> DashboardListListResponse {
        DashboardListListResponse {
            dashboard_lists: None,
        }
    }

    pub fn dashboard_lists(mut self, value: Vec<crate::datadogV1::model::DashboardList>) -> Self {
        self.dashboard_lists = Some(value);
        self
    }
}

impl Default for DashboardListListResponse {
    fn default() -> Self {
        Self::new()
    }
}
