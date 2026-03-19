// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Options returned in deployment rule responses representing either faulty deployment detection or monitor options. Faulty deployment detection responses always include `excluded_resources`, making the two variants unambiguous.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DeploymentRulesOptionsResponse {
    DeploymentRuleOptionsFaultyDeploymentDetectionResponse(
        Box<crate::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetectionResponse>,
    ),
    DeploymentRuleOptionsMonitor(Box<crate::datadogV2::model::DeploymentRuleOptionsMonitor>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DeploymentRulesOptionsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetectionResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DeploymentRulesOptionsResponse::DeploymentRuleOptionsFaultyDeploymentDetectionResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DeploymentRuleOptionsMonitor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DeploymentRulesOptionsResponse::DeploymentRuleOptionsMonitor(_v));
            }
        }

        return Ok(DeploymentRulesOptionsResponse::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
