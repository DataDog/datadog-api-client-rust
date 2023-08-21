// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsGrokParserType {
    #[serde(rename = "grok-parser")]
	GROK_PARSER,
}

impl ToString for LogsGrokParserType {
    fn to_string(&self) -> String {
        match self {
            Self::GROK_PARSER => String::from("grok-parser"),
        }
    }
}

impl Default for LogsGrokParserType {
    fn default() -> LogsGrokParserType {
        Self::GROK_PARSER
    }
}
