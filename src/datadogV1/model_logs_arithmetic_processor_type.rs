// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsArithmeticProcessorType {
    #[serde(rename = "arithmetic-processor")]
	ARITHMETIC_PROCESSOR,
}

impl ToString for LogsArithmeticProcessorType {
    fn to_string(&self) -> String {
        match self {
            Self::ARITHMETIC_PROCESSOR => String::from("arithmetic-processor"),
        }
    }
}

impl Default for LogsArithmeticProcessorType {
    fn default() -> LogsArithmeticProcessorType {
        Self::ARITHMETIC_PROCESSOR
    }
}
