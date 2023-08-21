// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsStatusRemapperType {
    #[serde(rename = "status-remapper")]
	STATUS_REMAPPER,
}

impl ToString for LogsStatusRemapperType {
    fn to_string(&self) -> String {
        match self {
            Self::STATUS_REMAPPER => String::from("status-remapper"),
        }
    }
}

impl Default for LogsStatusRemapperType {
    fn default() -> LogsStatusRemapperType {
        Self::STATUS_REMAPPER
    }
}
