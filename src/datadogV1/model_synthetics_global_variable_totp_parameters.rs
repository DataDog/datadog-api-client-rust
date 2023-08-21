// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableTOTPParameters {
    /// Number of digits for the OTP code.
    #[serde(rename = "digits", skip_serializing_if = "Option::is_none")]
    pub digits: i32,
    /// Interval for which to refresh the token (in seconds).
    #[serde(rename = "refresh_interval", skip_serializing_if = "Option::is_none")]
    pub refresh_interval: i32,
}

