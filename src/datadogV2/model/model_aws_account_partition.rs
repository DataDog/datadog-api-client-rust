// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AWSAccountPartition {
    AWS,
    AWS_CN,
    AWS_US_GOV,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AWSAccountPartition {
    fn to_string(&self) -> String {
        match self {
            Self::AWS => String::from("aws"),
            Self::AWS_CN => String::from("aws-cn"),
            Self::AWS_US_GOV => String::from("aws-us-gov"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AWSAccountPartition {
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

impl<'de> Deserialize<'de> for AWSAccountPartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aws" => Self::AWS,
            "aws-cn" => Self::AWS_CN,
            "aws-us-gov" => Self::AWS_US_GOV,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
