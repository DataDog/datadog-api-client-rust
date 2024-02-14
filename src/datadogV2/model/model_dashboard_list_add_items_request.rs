// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request containing a list of dashboards to add.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListAddItemsRequest {
    /// List of dashboards to add the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Option<Vec<crate::datadogV2::model::DashboardListItemRequest>>,
}

impl DashboardListAddItemsRequest {
    pub fn new() -> DashboardListAddItemsRequest {
        DashboardListAddItemsRequest { dashboards: None }
    }

    pub fn dashboards(
        &mut self,
        value: Vec<crate::datadogV2::model::DashboardListItemRequest>,
    ) -> &mut Self {
        self.dashboards = Some(value);
        self
    }
}

impl Default for DashboardListAddItemsRequest {
    fn default() -> Self {
        Self::new()
    }
}
