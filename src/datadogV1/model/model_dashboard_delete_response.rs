// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response from the delete dashboard call.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardDeleteResponse {
    /// ID of the deleted dashboard.
    #[serde(rename = "deleted_dashboard_id")]
    pub deleted_dashboard_id: Option<String>,
}

impl DashboardDeleteResponse {
    pub fn new() -> DashboardDeleteResponse {
        DashboardDeleteResponse {
            deleted_dashboard_id: None,
        }
    }
}
impl Default for DashboardDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}
