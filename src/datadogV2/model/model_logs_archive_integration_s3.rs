// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The S3 Archive's integration destination. You must provide one of the following: `access_key_id` alone, or both `account_id` and `role_name` together.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsArchiveIntegrationS3 {
    LogsArchiveIntegrationS3AccessKey(
        Box<crate::datadogV2::model::LogsArchiveIntegrationS3AccessKey>,
    ),
    LogsArchiveIntegrationS3Role(Box<crate::datadogV2::model::LogsArchiveIntegrationS3Role>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsArchiveIntegrationS3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LogsArchiveIntegrationS3AccessKey>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArchiveIntegrationS3::LogsArchiveIntegrationS3AccessKey(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LogsArchiveIntegrationS3Role>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArchiveIntegrationS3::LogsArchiveIntegrationS3Role(_v));
            }
        }

        return Ok(LogsArchiveIntegrationS3::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
