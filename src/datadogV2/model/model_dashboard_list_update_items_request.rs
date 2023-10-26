// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListUpdateItemsRequest {
    /// List of dashboards to update the dashboard list to.
    #[serde(rename = "dashboards", skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>>,
}

impl DashboardListUpdateItemsRequest {
    /// Request containing the list of dashboards to update to.
    pub fn new() -> DashboardListUpdateItemsRequest {
        DashboardListUpdateItemsRequest { dashboards: None }
    }
}
