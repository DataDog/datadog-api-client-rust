// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultShortResult {
    /// Describes if the test run has passed or failed.
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: bool,
    /// Object containing all metrics and their values collected for a Synthetic API test.
See the [Synthetic Monitoring Metrics documentation](https://docs.datadoghq.com/synthetics/metrics/).
    #[serde(rename = "timings", skip_serializing_if = "Option::is_none")]
    pub timings: SyntheticsTiming,
}

