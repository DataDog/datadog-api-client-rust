// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssuePlatform {
    ANDROID,
    BACKEND,
    BROWSER,
    FLUTTER,
    IOS,
    REACT_NATIVE,
    ROKU,
    UNKNOWN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IssuePlatform {
    fn to_string(&self) -> String {
        match self {
            Self::ANDROID => String::from("ANDROID"),
            Self::BACKEND => String::from("BACKEND"),
            Self::BROWSER => String::from("BROWSER"),
            Self::FLUTTER => String::from("FLUTTER"),
            Self::IOS => String::from("IOS"),
            Self::REACT_NATIVE => String::from("REACT_NATIVE"),
            Self::ROKU => String::from("ROKU"),
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IssuePlatform {
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

impl<'de> Deserialize<'de> for IssuePlatform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ANDROID" => Self::ANDROID,
            "BACKEND" => Self::BACKEND,
            "BROWSER" => Self::BROWSER,
            "FLUTTER" => Self::FLUTTER,
            "IOS" => Self::IOS,
            "REACT_NATIVE" => Self::REACT_NATIVE,
            "ROKU" => Self::ROKU,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
