// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListItems {
    /// List of dashboards in the dashboard list.
    #[serde(rename = "dashboards")]
    pub dashboards: Vec<crate::datadogV2::model::DashboardListItem>,
    /// Number of dashboards in the dashboard list.
    #[serde(rename = "total")]
    pub total: Option<i64>,
}

impl DashboardListItems {
    /// Dashboards within a list.
    pub fn new(dashboards: Vec<crate::datadogV2::model::DashboardListItem>) -> DashboardListItems {
        DashboardListItems {
            dashboards,
            total: None,
        }
    }
}
