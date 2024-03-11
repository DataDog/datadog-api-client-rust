// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Incident integration metadata's metadata attribute.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncidentIntegrationMetadataMetadata {
    SlackIntegrationMetadata(Box<crate::datadogV2::model::SlackIntegrationMetadata>),
    JiraIntegrationMetadata(Box<crate::datadogV2::model::JiraIntegrationMetadata>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for IncidentIntegrationMetadataMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SlackIntegrationMetadata>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentIntegrationMetadataMetadata::SlackIntegrationMetadata(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::JiraIntegrationMetadata>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentIntegrationMetadataMetadata::JiraIntegrationMetadata(_v));
            }
        }

        return Ok(IncidentIntegrationMetadataMetadata::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
