// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper around an array of [JSON Patch](<https://jsonpatch.com>) operations to perform on the test
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPatchTestBody {
    /// Array of [JSON Patch](<https://jsonpatch.com>) operations to perform on the test
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::SyntheticsPatchTestOperation>>,
}

impl SyntheticsPatchTestBody {
    pub fn new() -> SyntheticsPatchTestBody {
        SyntheticsPatchTestBody { data: None }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsPatchTestOperation>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for SyntheticsPatchTestBody {
    fn default() -> Self {
        Self::new()
    }
}
