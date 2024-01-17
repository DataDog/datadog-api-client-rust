// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Create custom grok rules to parse the full message or [a specific attribute of your raw event](https://docs.datadoghq.com/logs/log_configuration/parsing/#advanced-settings).
/// For more information, see the [parsing section](https://docs.datadoghq.com/logs/log_configuration/parsing).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGrokParser {
    /// Set of rules for the grok parser.
    #[serde(rename = "grok")]
    pub grok: Box<crate::datadogV1::model::LogsGrokParserRules>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of sample logs to test this grok parser.
    #[serde(rename = "samples")]
    pub samples: Option<Vec<String>>,
    /// Name of the log attribute to parse.
    #[serde(rename = "source")]
    pub source: String,
    /// Type of logs grok parser.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsGrokParserType,
}

impl LogsGrokParser {
    pub fn new(
        grok: Box<crate::datadogV1::model::LogsGrokParserRules>,
        source: String,
        type_: crate::datadogV1::model::LogsGrokParserType,
    ) -> LogsGrokParser {
        LogsGrokParser {
            grok,
            is_enabled: None,
            name: None,
            samples: None,
            source,
            type_,
        }
    }
}
