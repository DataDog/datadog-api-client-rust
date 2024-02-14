// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing an array of Synthetic global variables.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsListGlobalVariablesResponse {
    /// Array of Synthetic global variables.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsGlobalVariable>>,
}

impl SyntheticsListGlobalVariablesResponse {
    pub fn new() -> SyntheticsListGlobalVariablesResponse {
        SyntheticsListGlobalVariablesResponse { variables: None }
    }

    pub fn variables(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsGlobalVariable>,
    ) -> &mut Self {
        self.variables = Some(value);
        self
    }
}

impl Default for SyntheticsListGlobalVariablesResponse {
    fn default() -> Self {
        Self::new()
    }
}
