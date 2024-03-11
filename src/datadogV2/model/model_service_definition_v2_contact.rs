// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Service owner's contacts information.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ServiceDefinitionV2Contact {
    ServiceDefinitionV2Email(Box<crate::datadogV2::model::ServiceDefinitionV2Email>),
    ServiceDefinitionV2Slack(Box<crate::datadogV2::model::ServiceDefinitionV2Slack>),
    ServiceDefinitionV2MSTeams(Box<crate::datadogV2::model::ServiceDefinitionV2MSTeams>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for ServiceDefinitionV2Contact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Email>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Contact::ServiceDefinitionV2Email(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Slack>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Contact::ServiceDefinitionV2Slack(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2MSTeams>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionV2Contact::ServiceDefinitionV2MSTeams(_v));
            }
        }

        return Ok(ServiceDefinitionV2Contact::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
