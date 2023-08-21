// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListDeleteItemsResponse {
    /// List of dashboards deleted from the dashboard list.
    #[serde(rename = "deleted_dashboards_from_list", skip_serializing_if = "Option::is_none")]
    pub deleted_dashboards_from_list: Vec<DashboardListItemResponse>,
}

