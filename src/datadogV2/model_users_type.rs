// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsersType {
    #[serde(rename = "users")]
	USERS,
}

impl ToString for UsersType {
    fn to_string(&self) -> String {
        match self {
            Self::USERS => String::from("users"),
        }
    }
}

impl Default for UsersType {
    fn default() -> UsersType {
        Self::USERS
    }
}
