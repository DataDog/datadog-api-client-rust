// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An application key response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyResponse {
    /// An application key with its associated metadata.
    #[serde(rename = "application_key")]
    pub application_key: Option<crate::datadogV1::model::ApplicationKey>,
}

impl ApplicationKeyResponse {
    pub fn new() -> ApplicationKeyResponse {
        ApplicationKeyResponse {
            application_key: None,
        }
    }

    pub fn application_key(mut self, value: crate::datadogV1::model::ApplicationKey) -> Self {
        self.application_key = Some(value);
        self
    }
}

impl Default for ApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}
