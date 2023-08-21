// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpansAggregateResponseStatus {
    #[serde(rename = "done")]
	DONE,
    #[serde(rename = "timeout")]
	TIMEOUT,
}

impl ToString for SpansAggregateResponseStatus {
    fn to_string(&self) -> String {
        match self {
            Self::DONE => String::from("done"),
            Self::TIMEOUT => String::from("timeout"),
        }
    }
}

impl Default for SpansAggregateResponseStatus {
    fn default() -> SpansAggregateResponseStatus {
        Self::DONE
    }
}
