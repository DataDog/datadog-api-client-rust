// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response for Cloud Cost activity.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudCostActivityResponse {
    /// Cloud Cost Activity.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::CloudCostActivity>,
}

impl CloudCostActivityResponse {
    pub fn new() -> CloudCostActivityResponse {
        CloudCostActivityResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::CloudCostActivity) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for CloudCostActivityResponse {
    fn default() -> Self {
        Self::new()
    }
}
