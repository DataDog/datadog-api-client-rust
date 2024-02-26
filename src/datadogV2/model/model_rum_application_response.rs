// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationResponse {
    /// RUM application.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::RUMApplication>,
}

impl RUMApplicationResponse {
    pub fn new() -> RUMApplicationResponse {
        RUMApplicationResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::RUMApplication) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for RUMApplicationResponse {
    fn default() -> Self {
        Self::new()
    }
}
