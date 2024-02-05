// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsDeviceID {
    #[serde(rename = "laptop_large")]
    LAPTOP_LARGE,
    #[serde(rename = "tablet")]
    TABLET,
    #[serde(rename = "mobile_small")]
    MOBILE_SMALL,
    #[serde(rename = "chrome.laptop_large")]
    CHROME_LAPTOP_LARGE,
    #[serde(rename = "chrome.tablet")]
    CHROME_TABLET,
    #[serde(rename = "chrome.mobile_small")]
    CHROME_MOBILE_SMALL,
    #[serde(rename = "firefox.laptop_large")]
    FIREFOX_LAPTOP_LARGE,
    #[serde(rename = "firefox.tablet")]
    FIREFOX_TABLET,
    #[serde(rename = "firefox.mobile_small")]
    FIREFOX_MOBILE_SMALL,
    #[serde(rename = "edge.laptop_large")]
    EDGE_LAPTOP_LARGE,
    #[serde(rename = "edge.tablet")]
    EDGE_TABLET,
    #[serde(rename = "edge.mobile_small")]
    EDGE_MOBILE_SMALL,
}

impl ToString for SyntheticsDeviceID {
    fn to_string(&self) -> String {
        match self {
            Self::LAPTOP_LARGE => String::from("laptop_large"),
            Self::TABLET => String::from("tablet"),
            Self::MOBILE_SMALL => String::from("mobile_small"),
            Self::CHROME_LAPTOP_LARGE => String::from("chrome.laptop_large"),
            Self::CHROME_TABLET => String::from("chrome.tablet"),
            Self::CHROME_MOBILE_SMALL => String::from("chrome.mobile_small"),
            Self::FIREFOX_LAPTOP_LARGE => String::from("firefox.laptop_large"),
            Self::FIREFOX_TABLET => String::from("firefox.tablet"),
            Self::FIREFOX_MOBILE_SMALL => String::from("firefox.mobile_small"),
            Self::EDGE_LAPTOP_LARGE => String::from("edge.laptop_large"),
            Self::EDGE_TABLET => String::from("edge.tablet"),
            Self::EDGE_MOBILE_SMALL => String::from("edge.mobile_small"),
        }
    }
}
