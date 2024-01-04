// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team permission settings response
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamPermissionSettingsResponse {
    /// Team permission settings response data
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::TeamPermissionSetting>>,
}

impl TeamPermissionSettingsResponse {
    pub fn new() -> TeamPermissionSettingsResponse {
        TeamPermissionSettingsResponse { data: None }
    }
}