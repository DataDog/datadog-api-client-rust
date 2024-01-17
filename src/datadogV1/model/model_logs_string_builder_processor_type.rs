// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsStringBuilderProcessorType {
    #[serde(rename = "string-builder-processor")]
    STRING_BUILDER_PROCESSOR,
}

impl ToString for LogsStringBuilderProcessorType {
    fn to_string(&self) -> String {
        match self {
            Self::STRING_BUILDER_PROCESSOR => String::from("string-builder-processor"),
        }
    }
}
