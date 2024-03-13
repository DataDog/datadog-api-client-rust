// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team links response
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLinksResponse {
    /// Team links response data
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::TeamLink>>,
}

impl TeamLinksResponse {
    pub fn new() -> TeamLinksResponse {
        TeamLinksResponse { data: None }
    }

    pub fn data(mut self, value: Vec<crate::datadogV2::model::TeamLink>) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for TeamLinksResponse {
    fn default() -> Self {
        Self::new()
    }
}
