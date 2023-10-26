// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListDeleteItemsResponse {
    /// List of dashboards deleted from the dashboard list.
    #[serde(rename = "deleted_dashboards_from_list", skip_serializing_if = "Option::is_none")]
    pub deleted_dashboards_from_list: Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
}

impl DashboardListDeleteItemsResponse {
    /// Response containing a list of deleted dashboards.
    pub fn new() -> DashboardListDeleteItemsResponse {
        DashboardListDeleteItemsResponse {
            deleted_dashboards_from_list: None,
        }
    }
}
