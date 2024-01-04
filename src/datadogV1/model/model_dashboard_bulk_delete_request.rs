// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Dashboard bulk delete request body.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardBulkDeleteRequest {
    /// List of dashboard bulk action request data objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV1::model::DashboardBulkActionData>,
}

impl DashboardBulkDeleteRequest {
    pub fn new(
        data: Vec<crate::datadogV1::model::DashboardBulkActionData>,
    ) -> DashboardBulkDeleteRequest {
        DashboardBulkDeleteRequest { data }
    }
}