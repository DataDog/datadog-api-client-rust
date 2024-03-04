// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to start or pause an existing Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsUpdateTestPauseStatusPayload {
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "new_status")]
    pub new_status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
}

impl SyntheticsUpdateTestPauseStatusPayload {
    pub fn new() -> SyntheticsUpdateTestPauseStatusPayload {
        SyntheticsUpdateTestPauseStatusPayload { new_status: None }
    }

    pub fn new_status(mut self, value: crate::datadogV1::model::SyntheticsTestPauseStatus) -> Self {
        self.new_status = Some(value);
        self
    }
}

impl Default for SyntheticsUpdateTestPauseStatusPayload {
    fn default() -> Self {
        Self::new()
    }
}
