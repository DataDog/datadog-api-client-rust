// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing a list of added dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListAddItemsResponse {
    /// List of dashboards added to the dashboard list.
    #[serde(rename = "added_dashboards_to_list")]
    pub added_dashboards_to_list: Option<Vec<crate::datadogV2::model::DashboardListItemResponse>>,
}

impl DashboardListAddItemsResponse {
    pub fn new() -> DashboardListAddItemsResponse {
        DashboardListAddItemsResponse {
            added_dashboards_to_list: None,
        }
    }
}