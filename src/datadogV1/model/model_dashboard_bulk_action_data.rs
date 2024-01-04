// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Dashboard bulk action request data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardBulkActionData {
    /// Dashboard resource ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Dashboard resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::DashboardResourceType,
}

impl DashboardBulkActionData {
    pub fn new(
        id: String,
        type_: crate::datadogV1::model::DashboardResourceType,
    ) -> DashboardBulkActionData {
        DashboardBulkActionData { id, type_ }
    }
}
