// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing information about the tests triggered.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerCITestsResponse {
    /// The public ID of the batch triggered.
    #[serde(
        rename = "batch_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub batch_id: Option<Option<String>>,
    /// List of Synthetic locations.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::datadogV1::model::SyntheticsTriggerCITestLocation>>,
    /// Information about the tests runs.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsTriggerCITestRunResult>>,
    /// The public IDs of the Synthetic test triggered.
    #[serde(rename = "triggered_check_ids")]
    pub triggered_check_ids: Option<Vec<String>>,
}

impl SyntheticsTriggerCITestsResponse {
    pub fn new() -> SyntheticsTriggerCITestsResponse {
        SyntheticsTriggerCITestsResponse {
            batch_id: None,
            locations: None,
            results: None,
            triggered_check_ids: None,
        }
    }
}
impl Default for SyntheticsTriggerCITestsResponse {
    fn default() -> Self {
        Self::new()
    }
}
