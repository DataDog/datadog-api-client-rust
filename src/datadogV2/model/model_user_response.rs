// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing information about a single user.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    /// User object returned by the API.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::User>>,
    /// Array of objects related to the user.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::UserResponseIncludedItem>>,
}

impl UserResponse {
    pub fn new() -> UserResponse {
        UserResponse {
            data: None,
            included: None,
        }
    }
}