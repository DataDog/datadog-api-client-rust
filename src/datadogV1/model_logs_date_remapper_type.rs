// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsDateRemapperType {
    #[serde(rename = "date-remapper")]
	DATE_REMAPPER,
}

impl ToString for LogsDateRemapperType {
    fn to_string(&self) -> String {
        match self {
            Self::DATE_REMAPPER => String::from("date-remapper"),
        }
    }
}

impl Default for LogsDateRemapperType {
    fn default() -> LogsDateRemapperType {
        Self::DATE_REMAPPER
    }
}
