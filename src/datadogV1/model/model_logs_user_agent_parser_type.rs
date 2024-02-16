// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsUserAgentParserType {
    #[serde(rename = "user-agent-parser")]
    USER_AGENT_PARSER,
}

impl ToString for LogsUserAgentParserType {
    fn to_string(&self) -> String {
        match self {
            Self::USER_AGENT_PARSER => String::from("user-agent-parser"),
        }
    }
}
