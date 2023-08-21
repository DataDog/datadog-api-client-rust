// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsURLParserType {
    #[serde(rename = "url-parser")]
	URL_PARSER,
}

impl ToString for LogsURLParserType {
    fn to_string(&self) -> String {
        match self {
            Self::URL_PARSER => String::from("url-parser"),
        }
    }
}

impl Default for LogsURLParserType {
    fn default() -> LogsURLParserType {
        Self::URL_PARSER
    }
}
