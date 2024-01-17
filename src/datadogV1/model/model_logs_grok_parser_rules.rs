// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Set of rules for the grok parser.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGrokParserRules {
    /// List of match rules for the grok parser, separated by a new line.
    #[serde(rename = "match_rules")]
    pub match_rules: String,
    /// List of support rules for the grok parser, separated by a new line.
    #[serde(rename = "support_rules")]
    pub support_rules: Option<String>,
}

impl LogsGrokParserRules {
    pub fn new(match_rules: String) -> LogsGrokParserRules {
        LogsGrokParserRules {
            match_rules,
            support_rules: None,
        }
    }
}
