// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An application key response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyListResponse {
    /// Array of application keys.
    #[serde(rename = "application_keys")]
    pub application_keys: Option<Vec<crate::datadogV1::model::ApplicationKey>>,
}

impl ApplicationKeyListResponse {
    pub fn new() -> ApplicationKeyListResponse {
        ApplicationKeyListResponse {
            application_keys: None,
        }
    }
}
