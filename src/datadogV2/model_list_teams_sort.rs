// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListTeamsSort {
    #[serde(rename = "name")]
	NAME,
    #[serde(rename = "-name")]
	_NAME,
    #[serde(rename = "user_count")]
	USER_COUNT,
    #[serde(rename = "-user_count")]
	_USER_COUNT,
}

impl ToString for ListTeamsSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::_NAME => String::from("-name"),
            Self::USER_COUNT => String::from("user_count"),
            Self::_USER_COUNT => String::from("-user_count"),
        }
    }
}

impl Default for ListTeamsSort {
    fn default() -> ListTeamsSort {
        Self::NAME
    }
}
