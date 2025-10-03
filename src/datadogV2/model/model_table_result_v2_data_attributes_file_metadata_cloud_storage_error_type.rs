// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TableResultV2DataAttributesFileMetadataCloudStorageErrorType {
    TABLE_SCHEMA_ERROR,
    FILE_FORMAT_ERROR,
    CONFIGURATION_ERROR,
    QUOTA_EXCEEDED,
    CONFLICT_ERROR,
    VALIDATION_ERROR,
    STATE_ERROR,
    OPERATION_ERROR,
    SYSTEM_ERROR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TableResultV2DataAttributesFileMetadataCloudStorageErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::TABLE_SCHEMA_ERROR => String::from("TABLE_SCHEMA_ERROR"),
            Self::FILE_FORMAT_ERROR => String::from("FILE_FORMAT_ERROR"),
            Self::CONFIGURATION_ERROR => String::from("CONFIGURATION_ERROR"),
            Self::QUOTA_EXCEEDED => String::from("QUOTA_EXCEEDED"),
            Self::CONFLICT_ERROR => String::from("CONFLICT_ERROR"),
            Self::VALIDATION_ERROR => String::from("VALIDATION_ERROR"),
            Self::STATE_ERROR => String::from("STATE_ERROR"),
            Self::OPERATION_ERROR => String::from("OPERATION_ERROR"),
            Self::SYSTEM_ERROR => String::from("SYSTEM_ERROR"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TableResultV2DataAttributesFileMetadataCloudStorageErrorType {
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

impl<'de> Deserialize<'de> for TableResultV2DataAttributesFileMetadataCloudStorageErrorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "TABLE_SCHEMA_ERROR" => Self::TABLE_SCHEMA_ERROR,
            "FILE_FORMAT_ERROR" => Self::FILE_FORMAT_ERROR,
            "CONFIGURATION_ERROR" => Self::CONFIGURATION_ERROR,
            "QUOTA_EXCEEDED" => Self::QUOTA_EXCEEDED,
            "CONFLICT_ERROR" => Self::CONFLICT_ERROR,
            "VALIDATION_ERROR" => Self::VALIDATION_ERROR,
            "STATE_ERROR" => Self::STATE_ERROR,
            "OPERATION_ERROR" => Self::OPERATION_ERROR,
            "SYSTEM_ERROR" => Self::SYSTEM_ERROR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
