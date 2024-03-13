// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsDeviceID {
    LAPTOP_LARGE,
    TABLET,
    MOBILE_SMALL,
    CHROME_LAPTOP_LARGE,
    CHROME_TABLET,
    CHROME_MOBILE_SMALL,
    FIREFOX_LAPTOP_LARGE,
    FIREFOX_TABLET,
    FIREFOX_MOBILE_SMALL,
    EDGE_LAPTOP_LARGE,
    EDGE_TABLET,
    EDGE_MOBILE_SMALL,
    UnparsedObject(crate::datadog::UnparsedObject),
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
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsDeviceID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsDeviceID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "laptop_large" => Self::LAPTOP_LARGE,
            "tablet" => Self::TABLET,
            "mobile_small" => Self::MOBILE_SMALL,
            "chrome.laptop_large" => Self::CHROME_LAPTOP_LARGE,
            "chrome.tablet" => Self::CHROME_TABLET,
            "chrome.mobile_small" => Self::CHROME_MOBILE_SMALL,
            "firefox.laptop_large" => Self::FIREFOX_LAPTOP_LARGE,
            "firefox.tablet" => Self::FIREFOX_TABLET,
            "firefox.mobile_small" => Self::FIREFOX_MOBILE_SMALL,
            "edge.laptop_large" => Self::EDGE_LAPTOP_LARGE,
            "edge.tablet" => Self::EDGE_TABLET,
            "edge.mobile_small" => Self::EDGE_MOBILE_SMALL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
