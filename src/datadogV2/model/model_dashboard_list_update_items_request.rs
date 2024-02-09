// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request containing the list of dashboards to update to.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListUpdateItemsRequest {
    /// List of dashboards to update the dashboard list to.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>>,
}

impl DashboardListUpdateItemsRequest {
    pub fn new() -> DashboardListUpdateItemsRequest {
        DashboardListUpdateItemsRequest { dashboards: None }
    }

    pub fn dashboards(
        &mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemRequest>,
    ) -> &mut Self {
        self.dashboards = Some(value);
        self
    }
}

impl Default for DashboardListUpdateItemsRequest {
    fn default() -> Self {
        Self::new()
    }
}
