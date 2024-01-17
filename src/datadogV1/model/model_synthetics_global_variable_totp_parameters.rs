// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Parameters for the TOTP/MFA variable
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableTOTPParameters {
    /// Number of digits for the OTP code.
    #[serde(rename = "digits")]
    pub digits: Option<i32>,
    /// Interval for which to refresh the token (in seconds).
    #[serde(rename = "refresh_interval")]
    pub refresh_interval: Option<i32>,
}

impl SyntheticsGlobalVariableTOTPParameters {
    pub fn new() -> SyntheticsGlobalVariableTOTPParameters {
        SyntheticsGlobalVariableTOTPParameters {
            digits: None,
            refresh_interval: None,
        }
    }
}
impl Default for SyntheticsGlobalVariableTOTPParameters {
    fn default() -> Self {
        Self::new()
    }
}
