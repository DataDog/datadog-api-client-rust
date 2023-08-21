// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogType {
    #[serde(rename = "log")]
	LOG,
}

impl ToString for LogType {
    fn to_string(&self) -> String {
        match self {
            Self::LOG => String::from("log"),
        }
    }
}

impl Default for LogType {
    fn default() -> LogType {
        Self::LOG
    }
}
