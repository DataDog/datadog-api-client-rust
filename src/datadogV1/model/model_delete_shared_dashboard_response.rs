// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing token of deleted shared dashboard.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSharedDashboardResponse {
    /// Token associated with the shared dashboard that was revoked.
    #[serde(rename = "deleted_public_dashboard_token")]
    pub deleted_public_dashboard_token: Option<String>,
}

impl DeleteSharedDashboardResponse {
    pub fn new() -> DeleteSharedDashboardResponse {
        DeleteSharedDashboardResponse {
            deleted_public_dashboard_token: None,
        }
    }

    pub fn with_deleted_public_dashboard_token(&mut self, value: String) -> &mut Self {
        self.deleted_public_dashboard_token = Some(value);
        self
    }
}
impl Default for DeleteSharedDashboardResponse {
    fn default() -> Self {
        Self::new()
    }
}
