// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsPlayingTab {
    #[serde(rename = "-1")]
	MAIN_TAB,
    #[serde(rename = "0")]
	NEW_TAB,
    #[serde(rename = "1")]
	TAB_1,
    #[serde(rename = "2")]
	TAB_2,
    #[serde(rename = "3")]
	TAB_3,
}

impl ToString for SyntheticsPlayingTab {
    fn to_string(&self) -> String {
        match self {
            Self::MAIN_TAB => String::from("-1"),
            Self::NEW_TAB => String::from("0"),
            Self::TAB_1 => String::from("1"),
            Self::TAB_2 => String::from("2"),
            Self::TAB_3 => String::from("3"),
        }
    }
}

impl Default for SyntheticsPlayingTab {
    fn default() -> SyntheticsPlayingTab {
        Self::MAIN_TAB
    }
}
