// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessRole {
    #[serde(rename = "st")]
	STANDARD,
    #[serde(rename = "adm")]
	ADMIN,
    #[serde(rename = "ro")]
	READ_ONLY,
    #[serde(rename = "ERROR")]
	ERROR,
}

impl ToString for AccessRole {
    fn to_string(&self) -> String {
        match self {
            Self::STANDARD => String::from("st"),
            Self::ADMIN => String::from("adm"),
            Self::READ_ONLY => String::from("ro"),
            Self::ERROR => String::from("ERROR"),
        }
    }
}

impl Default for AccessRole {
    fn default() -> AccessRole {
        Self::STANDARD
    }
}
