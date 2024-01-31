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

    pub fn with_batch_id(&mut self, value: Option<String>) -> &mut Self {
        self.batch_id = Some(value);
        self
    }

    pub fn with_locations(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTriggerCITestLocation>,
    ) -> &mut Self {
        self.locations = Some(value);
        self
    }

    pub fn with_results(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTriggerCITestRunResult>,
    ) -> &mut Self {
        self.results = Some(value);
        self
    }

    pub fn with_triggered_check_ids(&mut self, value: Vec<String>) -> &mut Self {
        self.triggered_check_ids = Some(value);
        self
    }
}
impl Default for SyntheticsTriggerCITestsResponse {
    fn default() -> Self {
        Self::new()
    }
}
