// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsNetworkAssertionOperator {
    IS,
    IS_NOT,
    LESS_THAN,
    LESS_THAN_OR_EQUAL,
    MORE_THAN,
    MORE_THAN_OR_EQUAL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SyntheticsNetworkAssertionOperator {
    fn to_string(&self) -> String {
        match self {
            Self::IS => String::from("is"),
            Self::IS_NOT => String::from("isNot"),
            Self::LESS_THAN => String::from("lessThan"),
            Self::LESS_THAN_OR_EQUAL => String::from("lessThanOrEqual"),
            Self::MORE_THAN => String::from("moreThan"),
            Self::MORE_THAN_OR_EQUAL => String::from("moreThanOrEqual"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsNetworkAssertionOperator {
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

impl<'de> Deserialize<'de> for SyntheticsNetworkAssertionOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "is" => Self::IS,
            "isNot" => Self::IS_NOT,
            "lessThan" => Self::LESS_THAN,
            "lessThanOrEqual" => Self::LESS_THAN_OR_EQUAL,
            "moreThan" => Self::MORE_THAN,
            "moreThanOrEqual" => Self::MORE_THAN_OR_EQUAL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
