// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Parsing options for variables to extract.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsParsingOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// Name of the variable to extract.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: Option<Box<crate::datadogV1::model::SyntheticsVariableParser>>,
    /// Determines whether or not the extracted value will be obfuscated.
    #[serde(rename = "secure")]
    pub secure: Option<bool>,
    /// Property of the Synthetic Test Response to use for a Synthetic global variable.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType>,
}

impl SyntheticsParsingOptions {
    pub fn new() -> SyntheticsParsingOptions {
        SyntheticsParsingOptions {
            field: None,
            name: None,
            parser: None,
            secure: None,
            type_: None,
        }
    }
}
