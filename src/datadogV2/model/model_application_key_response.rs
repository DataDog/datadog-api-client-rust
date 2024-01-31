// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for retrieving an application key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyResponse {
    /// Datadog application key.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::FullApplicationKey>,
    /// Array of objects related to the application key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>>,
}

impl ApplicationKeyResponse {
    pub fn new() -> ApplicationKeyResponse {
        ApplicationKeyResponse {
            data: None,
            included: None,
        }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::FullApplicationKey) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_included(
        &mut self,
        value: Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}
impl Default for ApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}
