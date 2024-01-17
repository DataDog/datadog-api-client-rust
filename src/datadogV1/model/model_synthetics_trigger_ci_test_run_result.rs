// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Information about a single test run.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerCITestRunResult {
    /// The device ID.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDeviceID>,
    /// The location ID of the test run.
    #[serde(rename = "location")]
    pub location: Option<i64>,
    /// The public ID of the Synthetic test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// ID of the result.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
}

impl SyntheticsTriggerCITestRunResult {
    pub fn new() -> SyntheticsTriggerCITestRunResult {
        SyntheticsTriggerCITestRunResult {
            device: None,
            location: None,
            public_id: None,
            result_id: None,
        }
    }
}
