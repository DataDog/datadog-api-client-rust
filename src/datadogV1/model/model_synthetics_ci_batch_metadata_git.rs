// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Git information.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadataGit {
    /// Branch name.
    #[serde(rename = "branch")]
    pub branch: Option<String>,
    /// The commit SHA.
    #[serde(rename = "commitSha")]
    pub commit_sha: Option<String>,
}

impl SyntheticsCIBatchMetadataGit {
    pub fn new() -> SyntheticsCIBatchMetadataGit {
        SyntheticsCIBatchMetadataGit {
            branch: None,
            commit_sha: None,
        }
    }

    pub fn branch(&mut self, value: String) -> &mut Self {
        self.branch = Some(value);
        self
    }

    pub fn commit_sha(&mut self, value: String) -> &mut Self {
        self.commit_sha = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadataGit {
    fn default() -> Self {
        Self::new()
    }
}
