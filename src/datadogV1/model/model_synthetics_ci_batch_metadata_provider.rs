// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of the CI provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCIBatchMetadataProvider {
    /// Name of the CI provider.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl SyntheticsCIBatchMetadataProvider {
    pub fn new() -> SyntheticsCIBatchMetadataProvider {
        SyntheticsCIBatchMetadataProvider { name: None }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadataProvider {
    fn default() -> Self {
        Self::new()
    }
}
