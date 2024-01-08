// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing a list of deleted dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListDeleteItemsResponse {
    /// List of dashboards deleted from the dashboard list.
    #[serde(rename = "deleted_dashboards_from_list")]
    pub deleted_dashboards_from_list:
        Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
}

impl DashboardListDeleteItemsResponse {
    pub fn new() -> DashboardListDeleteItemsResponse {
        DashboardListDeleteItemsResponse {
            deleted_dashboards_from_list: None,
        }
    }
}
impl Default for DashboardListDeleteItemsResponse {
    fn default() -> Self {
        Self::new()
    }
}
