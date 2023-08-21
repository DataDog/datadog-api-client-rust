// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DowntimeStatus {
    #[serde(rename = "active")]
	ACTIVE,
    #[serde(rename = "canceled")]
	CANCELED,
    #[serde(rename = "ended")]
	ENDED,
    #[serde(rename = "scheduled")]
	SCHEDULED,
}

impl ToString for DowntimeStatus {
    fn to_string(&self) -> String {
        match self {
            Self::ACTIVE => String::from("active"),
            Self::CANCELED => String::from("canceled"),
            Self::ENDED => String::from("ended"),
            Self::SCHEDULED => String::from("scheduled"),
        }
    }
}

impl Default for DowntimeStatus {
    fn default() -> DowntimeStatus {
        Self::ACTIVE
    }
}
