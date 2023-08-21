// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerCITestRunResult {
    /// The device ID.
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: SyntheticsDeviceID,
    /// The location ID of the test run.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: i64,
    /// The public ID of the Synthetic test.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// ID of the result.
    #[serde(rename = "result_id", skip_serializing_if = "Option::is_none")]
    pub result_id: String,
}

