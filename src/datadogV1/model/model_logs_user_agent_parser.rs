// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The User-Agent parser takes a User-Agent attribute and extracts the OS, browser, device, and other user data.
/// It recognizes major bots like the Google Bot, Yahoo Slurp, and Bing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsUserAgentParser {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Define if the source attribute is URL encoded or not.
    #[serde(rename = "is_encoded")]
    pub is_encoded: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources")]
    pub sources: Vec<String>,
    /// Name of the parent attribute that contains all the extracted details from the `sources`.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs User-Agent parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsUserAgentParserType,
}

impl LogsUserAgentParser {
    pub fn new(
        sources: Vec<String>,
        target: String,
        type_: crate::datadogV1::model::LogsUserAgentParserType,
    ) -> LogsUserAgentParser {
        LogsUserAgentParser {
            is_enabled: None,
            is_encoded: None,
            name: None,
            sources,
            target,
            type_,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn is_encoded(&mut self, value: bool) -> &mut Self {
        self.is_encoded = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
