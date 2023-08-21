// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventsSort {
    #[serde(rename = "timestamp")]
	TIMESTAMP_ASCENDING,
    #[serde(rename = "-timestamp")]
	TIMESTAMP_DESCENDING,
}

impl ToString for EventsSort {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESTAMP_ASCENDING => String::from("timestamp"),
            Self::TIMESTAMP_DESCENDING => String::from("-timestamp"),
        }
    }
}

impl Default for EventsSort {
    fn default() -> EventsSort {
        Self::TIMESTAMP_ASCENDING
    }
}
