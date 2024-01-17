// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A dashboard within a list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardListItemResponse {
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the dashboard.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DashboardType,
}

impl DashboardListItemResponse {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::DashboardType,
    ) -> DashboardListItemResponse {
        DashboardListItemResponse { id, type_ }
    }
}
