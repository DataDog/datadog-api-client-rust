// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsMobileStepParamsElementUserLocatorValuesItemsType {
    ACCESSIBILITY_ID,
    ID,
    IOS_PREDICATE_STRING,
    IOS_CLASS_CHAIN,
    XPATH,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsMobileStepParamsElementUserLocatorValuesItemsType {
    fn to_string(&self) -> String {
        match self {
            Self::ACCESSIBILITY_ID => String::from("accessibility-id"),
            Self::ID => String::from("id"),
            Self::IOS_PREDICATE_STRING => String::from("ios-predicate-string"),
            Self::IOS_CLASS_CHAIN => String::from("ios-class-chain"),
            Self::XPATH => String::from("xpath"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsMobileStepParamsElementUserLocatorValuesItemsType {
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

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsElementUserLocatorValuesItemsType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "accessibility-id" => Self::ACCESSIBILITY_ID,
            "id" => Self::ID,
            "ios-predicate-string" => Self::IOS_PREDICATE_STRING,
            "ios-class-chain" => Self::IOS_CLASS_CHAIN,
            "xpath" => Self::XPATH,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
