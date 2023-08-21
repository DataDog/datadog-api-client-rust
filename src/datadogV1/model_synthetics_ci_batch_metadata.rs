// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadata {
    /// Description of the CI provider.
    #[serde(rename = "ci", skip_serializing_if = "Option::is_none")]
    pub ci: SyntheticsCIBatchMetadataCI,
    /// Git information.
    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: SyntheticsCIBatchMetadataGit,
}

