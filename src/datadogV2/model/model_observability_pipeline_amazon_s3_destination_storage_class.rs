// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineAmazonS3DestinationStorageClass {
    STANDARD,
    REDUCED_REDUNDANCY,
    INTELLIGENT_TIERING,
    STANDARD_IA,
    EXPRESS_ONEZONE,
    ONEZONE_IA,
    GLACIER,
    GLACIER_IR,
    DEEP_ARCHIVE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineAmazonS3DestinationStorageClass {
    fn to_string(&self) -> String {
        match self {
            Self::STANDARD => String::from("STANDARD"),
            Self::REDUCED_REDUNDANCY => String::from("REDUCED_REDUNDANCY"),
            Self::INTELLIGENT_TIERING => String::from("INTELLIGENT_TIERING"),
            Self::STANDARD_IA => String::from("STANDARD_IA"),
            Self::EXPRESS_ONEZONE => String::from("EXPRESS_ONEZONE"),
            Self::ONEZONE_IA => String::from("ONEZONE_IA"),
            Self::GLACIER => String::from("GLACIER"),
            Self::GLACIER_IR => String::from("GLACIER_IR"),
            Self::DEEP_ARCHIVE => String::from("DEEP_ARCHIVE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineAmazonS3DestinationStorageClass {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3DestinationStorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "STANDARD" => Self::STANDARD,
            "REDUCED_REDUNDANCY" => Self::REDUCED_REDUNDANCY,
            "INTELLIGENT_TIERING" => Self::INTELLIGENT_TIERING,
            "STANDARD_IA" => Self::STANDARD_IA,
            "EXPRESS_ONEZONE" => Self::EXPRESS_ONEZONE,
            "ONEZONE_IA" => Self::ONEZONE_IA,
            "GLACIER" => Self::GLACIER,
            "GLACIER_IR" => Self::GLACIER_IR,
            "DEEP_ARCHIVE" => Self::DEEP_ARCHIVE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
