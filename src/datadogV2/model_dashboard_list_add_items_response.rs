// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListAddItemsResponse {
    /// List of dashboards added to the dashboard list.
    #[serde(rename = "added_dashboards_to_list", skip_serializing_if = "Option::is_none")]
    pub added_dashboards_to_list: Vec<DashboardListItemResponse>,
}

