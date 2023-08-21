// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerCITestsResponse {
    /// The public ID of the batch triggered.
    #[serde(rename = "batch_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub batch_id: Option<String>,
    /// List of Synthetic locations.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Vec<SyntheticsTriggerCITestLocation>,
    /// Information about the tests runs.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Vec<SyntheticsTriggerCITestRunResult>,
    /// The public IDs of the Synthetic test triggered.
    #[serde(rename = "triggered_check_ids", skip_serializing_if = "Option::is_none")]
    pub triggered_check_ids: Vec<String>,
}

