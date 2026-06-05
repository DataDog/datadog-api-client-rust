// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AWSCcmConfigValidationIssueCode {
    ISSUE_CODE_UNSPECIFIED,
    CREDENTIAL_ERROR,
    BUCKET_NAME_INVALID_GOVCLOUD,
    S3_LIST_PERMISSION_MISSING,
    S3_GET_PERMISSION_MISSING,
    S3_BUCKET_REGION_MISMATCH,
    S3_BUCKET_NOT_ACCESSIBLE,
    EXPORT_LIST_PERMISSION_MISSING,
    EXPORT_GET_PERMISSION_MISSING,
    EXPORT_NOT_FOUND,
    EXPORT_STATUS_UNHEALTHY,
    TIME_GRANULARITY_INVALID,
    FILE_FORMAT_INVALID,
    INCLUDE_RESOURCES_DISABLED,
    REFRESH_CADENCE_INVALID,
    OVERWRITE_MODE_INVALID,
    QUERY_STATEMENT_INVALID,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AWSCcmConfigValidationIssueCode {
    fn to_string(&self) -> String {
        match self {
            Self::ISSUE_CODE_UNSPECIFIED => String::from("ISSUE_CODE_UNSPECIFIED"),
            Self::CREDENTIAL_ERROR => String::from("CREDENTIAL_ERROR"),
            Self::BUCKET_NAME_INVALID_GOVCLOUD => String::from("BUCKET_NAME_INVALID_GOVCLOUD"),
            Self::S3_LIST_PERMISSION_MISSING => String::from("S3_LIST_PERMISSION_MISSING"),
            Self::S3_GET_PERMISSION_MISSING => String::from("S3_GET_PERMISSION_MISSING"),
            Self::S3_BUCKET_REGION_MISMATCH => String::from("S3_BUCKET_REGION_MISMATCH"),
            Self::S3_BUCKET_NOT_ACCESSIBLE => String::from("S3_BUCKET_NOT_ACCESSIBLE"),
            Self::EXPORT_LIST_PERMISSION_MISSING => String::from("EXPORT_LIST_PERMISSION_MISSING"),
            Self::EXPORT_GET_PERMISSION_MISSING => String::from("EXPORT_GET_PERMISSION_MISSING"),
            Self::EXPORT_NOT_FOUND => String::from("EXPORT_NOT_FOUND"),
            Self::EXPORT_STATUS_UNHEALTHY => String::from("EXPORT_STATUS_UNHEALTHY"),
            Self::TIME_GRANULARITY_INVALID => String::from("TIME_GRANULARITY_INVALID"),
            Self::FILE_FORMAT_INVALID => String::from("FILE_FORMAT_INVALID"),
            Self::INCLUDE_RESOURCES_DISABLED => String::from("INCLUDE_RESOURCES_DISABLED"),
            Self::REFRESH_CADENCE_INVALID => String::from("REFRESH_CADENCE_INVALID"),
            Self::OVERWRITE_MODE_INVALID => String::from("OVERWRITE_MODE_INVALID"),
            Self::QUERY_STATEMENT_INVALID => String::from("QUERY_STATEMENT_INVALID"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AWSCcmConfigValidationIssueCode {
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

impl<'de> Deserialize<'de> for AWSCcmConfigValidationIssueCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ISSUE_CODE_UNSPECIFIED" => Self::ISSUE_CODE_UNSPECIFIED,
            "CREDENTIAL_ERROR" => Self::CREDENTIAL_ERROR,
            "BUCKET_NAME_INVALID_GOVCLOUD" => Self::BUCKET_NAME_INVALID_GOVCLOUD,
            "S3_LIST_PERMISSION_MISSING" => Self::S3_LIST_PERMISSION_MISSING,
            "S3_GET_PERMISSION_MISSING" => Self::S3_GET_PERMISSION_MISSING,
            "S3_BUCKET_REGION_MISMATCH" => Self::S3_BUCKET_REGION_MISMATCH,
            "S3_BUCKET_NOT_ACCESSIBLE" => Self::S3_BUCKET_NOT_ACCESSIBLE,
            "EXPORT_LIST_PERMISSION_MISSING" => Self::EXPORT_LIST_PERMISSION_MISSING,
            "EXPORT_GET_PERMISSION_MISSING" => Self::EXPORT_GET_PERMISSION_MISSING,
            "EXPORT_NOT_FOUND" => Self::EXPORT_NOT_FOUND,
            "EXPORT_STATUS_UNHEALTHY" => Self::EXPORT_STATUS_UNHEALTHY,
            "TIME_GRANULARITY_INVALID" => Self::TIME_GRANULARITY_INVALID,
            "FILE_FORMAT_INVALID" => Self::FILE_FORMAT_INVALID,
            "INCLUDE_RESOURCES_DISABLED" => Self::INCLUDE_RESOURCES_DISABLED,
            "REFRESH_CADENCE_INVALID" => Self::REFRESH_CADENCE_INVALID,
            "OVERWRITE_MODE_INVALID" => Self::OVERWRITE_MODE_INVALID,
            "QUERY_STATEMENT_INVALID" => Self::QUERY_STATEMENT_INVALID,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
