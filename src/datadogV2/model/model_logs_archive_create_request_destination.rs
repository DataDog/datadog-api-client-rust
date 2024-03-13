// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// An archive's destination.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsArchiveCreateRequestDestination {
    LogsArchiveDestinationAzure(Box<crate::datadogV2::model::LogsArchiveDestinationAzure>),
    LogsArchiveDestinationGCS(Box<crate::datadogV2::model::LogsArchiveDestinationGCS>),
    LogsArchiveDestinationS3(Box<crate::datadogV2::model::LogsArchiveDestinationS3>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsArchiveCreateRequestDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LogsArchiveDestinationAzure>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArchiveCreateRequestDestination::LogsArchiveDestinationAzure(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LogsArchiveDestinationGCS>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArchiveCreateRequestDestination::LogsArchiveDestinationGCS(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LogsArchiveDestinationS3>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(LogsArchiveCreateRequestDestination::LogsArchiveDestinationS3(_v));
            }
        }

        return Ok(LogsArchiveCreateRequestDestination::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
