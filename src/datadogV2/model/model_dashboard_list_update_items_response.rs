// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListUpdateItemsResponse {
    /// List of dashboards in the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
}

impl DashboardListUpdateItemsResponse {
    /// Response containing a list of updated dashboards.
    pub fn new() -> DashboardListUpdateItemsResponse {
        DashboardListUpdateItemsResponse { dashboards: None }
    }
}
