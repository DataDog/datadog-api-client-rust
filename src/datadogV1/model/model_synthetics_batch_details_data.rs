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
    pub metadata: Option<Box<crate::datadogV1::model::SyntheticsCIBatchMetadata>>,
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
}
