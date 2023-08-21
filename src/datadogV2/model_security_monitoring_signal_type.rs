// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityMonitoringSignalType {
    #[serde(rename = "signal")]
	SIGNAL,
}

impl ToString for SecurityMonitoringSignalType {
    fn to_string(&self) -> String {
        match self {
            Self::SIGNAL => String::from("signal"),
        }
    }
}

impl Default for SecurityMonitoringSignalType {
    fn default() -> SecurityMonitoringSignalType {
        Self::SIGNAL
    }
}
