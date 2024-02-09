// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper object that contains the details of a batch.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBatchDetailsData {
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata>,
    /// List of results for the batch.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV1::model::SyntheticsBatchResult>>,
    /// Determines whether or not the batch has passed, failed, or is in progress.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsStatus>,
}

impl SyntheticsBatchDetailsData {
    pub fn new() -> SyntheticsBatchDetailsData {
        SyntheticsBatchDetailsData {
            metadata: None,
            results: None,
            status: None,
        }
    }

    pub fn metadata(
        &mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn results(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsBatchResult>,
    ) -> &mut Self {
        self.results = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV1::model::SyntheticsStatus) -> &mut Self {
        self.status = Some(value);
        self
    }
}

impl Default for SyntheticsBatchDetailsData {
    fn default() -> Self {
        Self::new()
    }
}
