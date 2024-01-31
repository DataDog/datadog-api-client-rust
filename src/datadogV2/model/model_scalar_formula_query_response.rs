// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A message containing one or more responses to scalar queries.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFormulaQueryResponse {
    /// A message containing the response to a scalar query.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::ScalarResponse>,
    /// An error generated when processing a request.
    #[serde(rename = "errors")]
    pub errors: Option<String>,
}

impl ScalarFormulaQueryResponse {
    pub fn new() -> ScalarFormulaQueryResponse {
        ScalarFormulaQueryResponse {
            data: None,
            errors: None,
        }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::ScalarResponse) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_errors(&mut self, value: String) -> &mut Self {
        self.errors = Some(value);
        self
    }
}
impl Default for ScalarFormulaQueryResponse {
    fn default() -> Self {
        Self::new()
    }
}
