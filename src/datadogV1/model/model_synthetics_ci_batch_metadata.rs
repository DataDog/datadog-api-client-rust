// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata for the Synthetic tests run.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadata {
    /// Description of the CI provider.
    #[serde(rename = "ci")]
    pub ci: Option<Box<crate::datadogV1::model::SyntheticsCIBatchMetadataCI>>,
    /// Git information.
    #[serde(rename = "git")]
    pub git: Option<Box<crate::datadogV1::model::SyntheticsCIBatchMetadataGit>>,
}

impl SyntheticsCIBatchMetadata {
    pub fn new() -> SyntheticsCIBatchMetadata {
        SyntheticsCIBatchMetadata {
            ci: None,
            git: None,
        }
    }
}
