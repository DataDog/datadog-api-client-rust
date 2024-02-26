// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of the parser to use for the global variable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsVariableParser {
    /// Type of parser for a Synthetic global variable from a synthetics test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsGlobalVariableParserType,
    /// Regex or JSON path used for the parser. Not used with type `raw`.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

impl SyntheticsVariableParser {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsGlobalVariableParserType,
    ) -> SyntheticsVariableParser {
        SyntheticsVariableParser { type_, value: None }
    }

    pub fn value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
}
