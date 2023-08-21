// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsGrokParserRules {
    /// List of match rules for the grok parser, separated by a new line.
    #[serde(rename = "match_rules", skip_serializing_if = "Option::is_none")]
    pub match_rules: String,
    /// List of support rules for the grok parser, separated by a new line.
    #[serde(rename = "support_rules", skip_serializing_if = "Option::is_none")]
    pub support_rules: String,
}

