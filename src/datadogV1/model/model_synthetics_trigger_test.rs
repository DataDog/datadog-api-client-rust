// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Test configuration for Synthetics
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerTest {
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata>,
    /// The public ID of the Synthetic test to trigger.
    #[serde(rename = "public_id")]
    pub public_id: String,
}

impl SyntheticsTriggerTest {
    pub fn new(public_id: String) -> SyntheticsTriggerTest {
        SyntheticsTriggerTest {
            metadata: None,
            public_id,
        }
    }

    pub fn with_metadata(
        &mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }
}
