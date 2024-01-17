// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This processor extracts query parameters and other important parameters from a URL.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsURLParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Normalize the ending slashes or not.
    #[serde(
        rename = "normalize_ending_slashes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub normalize_ending_slashes: Option<Option<bool>>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs URL parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsURLParserType,
}

impl LogsURLParser {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsURLParserType,
    ) -> LogsURLParser {
        LogsURLParser {
            is_enabled: None,
            name: None,
            normalize_ending_slashes: None,
            sources,
            target,
            type_,
        }
    }
}
