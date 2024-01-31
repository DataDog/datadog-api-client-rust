// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Parser options to use for retrieving a Synthetic global variable from a Synthetic test. Used in conjunction with `parse_test_public_id`.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableParseTestOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// When type is `local_variable`, name of the local variable to use to extract the value.
    #[serde(rename = "localVariableName")]
    pub local_variable_name: Option<String>,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: Option<crate::datadogV1::model::SyntheticsVariableParser>,
    /// Property of the Synthetic Test Response to use for a Synthetic global variable.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType,
}

impl SyntheticsGlobalVariableParseTestOptions {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType,
    ) -> SyntheticsGlobalVariableParseTestOptions {
        SyntheticsGlobalVariableParseTestOptions {
            field: None,
            local_variable_name: None,
            parser: None,
            type_,
        }
    }

    pub fn with_field(&mut self, value: String) -> &mut Self {
        self.field = Some(value);
        self
    }

    pub fn with_local_variable_name(&mut self, value: String) -> &mut Self {
        self.local_variable_name = Some(value);
        self
    }

    pub fn with_parser(
        &mut self,
        value: crate::datadogV1::model::SyntheticsVariableParser,
    ) -> &mut Self {
        self.parser = Some(value);
        self
    }
}
