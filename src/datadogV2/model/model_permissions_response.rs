// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PermissionsResponse {
    /// Array of permissions.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::Permission>>,
}

impl PermissionsResponse {
    /// Payload with API-returned permissions.
    pub fn new() -> PermissionsResponse {
        PermissionsResponse { data: None }
    }
}
