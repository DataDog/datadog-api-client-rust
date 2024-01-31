// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// CI visibility usage response
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCIVisibilityResponse {
    /// Response containing CI visibility usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageCIVisibilityHour>>,
}

impl UsageCIVisibilityResponse {
    pub fn new() -> UsageCIVisibilityResponse {
        UsageCIVisibilityResponse { usage: None }
    }

    pub fn with_usage(
        &mut self,
        value: Vec<crate::datadogV1::model::UsageCIVisibilityHour>,
    ) -> &mut Self {
        self.usage = Some(value);
        self
    }
}
impl Default for UsageCIVisibilityResponse {
    fn default() -> Self {
        Self::new()
    }
}
