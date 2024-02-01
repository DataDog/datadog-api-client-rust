// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Payload with API-returned permissions.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsResponse {
    /// Array of permissions.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::Permission>>,
}

impl PermissionsResponse {
    pub fn new() -> PermissionsResponse {
        PermissionsResponse { data: None }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::Permission>) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for PermissionsResponse {
    fn default() -> Self {
        Self::new()
    }
}
