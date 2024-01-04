// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Deleted dashboard details.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListDeleteResponse {
    /// ID of the deleted dashboard list.
    #[serde(rename = "deleted_dashboard_list_id")]
    pub deleted_dashboard_list_id: Option<i64>,
}

impl DashboardListDeleteResponse {
    pub fn new() -> DashboardListDeleteResponse {
        DashboardListDeleteResponse {
            deleted_dashboard_list_id: None,
        }
    }
}
