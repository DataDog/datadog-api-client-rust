// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing information about a created role.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCreateResponse {
    /// Role object returned by the API.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::RoleCreateResponseData>>,
}

impl RoleCreateResponse {
    pub fn new() -> RoleCreateResponse {
        RoleCreateResponse { data: None }
    }
}