// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of the CI pipeline.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadataPipeline {
    /// URL of the pipeline.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl SyntheticsCIBatchMetadataPipeline {
    pub fn new() -> SyntheticsCIBatchMetadataPipeline {
        SyntheticsCIBatchMetadataPipeline { url: None }
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadataPipeline {
    fn default() -> Self {
        Self::new()
    }
}
