// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Service owner's contacts information.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ServiceDefinitionV2Dot1Contact {
    ServiceDefinitionV2Dot1Email(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Email>),
    ServiceDefinitionV2Dot1Slack(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Slack>),
    ServiceDefinitionV2Dot1MSTeams(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1MSTeams>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Dot1Contact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Email>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Dot1Contact::ServiceDefinitionV2Dot1Email(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Slack>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Dot1Contact::ServiceDefinitionV2Dot1Slack(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Dot1MSTeams>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Dot1Contact::ServiceDefinitionV2Dot1MSTeams(_v));
            }
        }

        return Ok(ServiceDefinitionV2Dot1Contact::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
