// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DashboardListItemRequest {
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the dashboard.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DashboardType,
}

impl DashboardListItemRequest {
    /// A dashboard within a list.
    pub fn new(id: String, type_: crate::datadogV2::model::DashboardType) -> DashboardListItemRequest {
        DashboardListItemRequest { id: id, type_: type_ }
    }
}
