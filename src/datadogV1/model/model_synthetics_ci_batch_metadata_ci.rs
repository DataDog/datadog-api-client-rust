// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of the CI provider.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadataCI {
    /// Description of the CI pipeline.
    #[serde(rename = "pipeline")]
    pub pipeline: Option<Box<crate::datadogV1::model::SyntheticsCIBatchMetadataPipeline>>,
    /// Description of the CI provider.
    #[serde(rename = "provider")]
    pub provider: Option<Box<crate::datadogV1::model::SyntheticsCIBatchMetadataProvider>>,
}

impl SyntheticsCIBatchMetadataCI {
    pub fn new() -> SyntheticsCIBatchMetadataCI {
        SyntheticsCIBatchMetadataCI {
            pipeline: None,
            provider: None,
        }
    }
}