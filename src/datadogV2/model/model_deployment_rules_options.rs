// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Options for deployment rule response representing either faulty deployment detection or monitor options.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DeploymentRulesOptions {
    DeploymentRuleOptionsFaultyDeploymentDetection(
        Box<crate::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetection>,
    ),
    DeploymentRuleOptionsMonitor(Box<crate::datadogV2::model::DeploymentRuleOptionsMonitor>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DeploymentRulesOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DeploymentRuleOptionsFaultyDeploymentDetection>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    DeploymentRulesOptions::DeploymentRuleOptionsFaultyDeploymentDetection(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DeploymentRuleOptionsMonitor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DeploymentRulesOptions::DeploymentRuleOptionsMonitor(_v));
            }
        }

        return Ok(DeploymentRulesOptions::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
