// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogsArchiveStorageClassS3Type {
    STANDARD,
    STANDARD_IA,
    ONEZONE_IA,
    INTELLIGENT_TIERING,
    GLACIER_IR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for LogsArchiveStorageClassS3Type {
    fn to_string(&self) -> String {
        match self {
            Self::STANDARD => String::from("STANDARD"),
            Self::STANDARD_IA => String::from("STANDARD_IA"),
            Self::ONEZONE_IA => String::from("ONEZONE_IA"),
            Self::INTELLIGENT_TIERING => String::from("INTELLIGENT_TIERING"),
            Self::GLACIER_IR => String::from("GLACIER_IR"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for LogsArchiveStorageClassS3Type {
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

impl<'de> Deserialize<'de> for LogsArchiveStorageClassS3Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "STANDARD" => Self::STANDARD,
            "STANDARD_IA" => Self::STANDARD_IA,
            "ONEZONE_IA" => Self::ONEZONE_IA,
            "INTELLIGENT_TIERING" => Self::INTELLIGENT_TIERING,
            "GLACIER_IR" => Self::GLACIER_IR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
