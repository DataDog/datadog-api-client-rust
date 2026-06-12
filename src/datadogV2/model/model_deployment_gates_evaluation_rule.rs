// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A rule to evaluate as part of a deployment gate evaluation.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DeploymentGatesEvaluationRule {
    DeploymentGatesMonitorRule(Box<crate::datadogV2::model::DeploymentGatesMonitorRule>),
    DeploymentGatesFDDRule(Box<crate::datadogV2::model::DeploymentGatesFDDRule>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DeploymentGatesEvaluationRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DeploymentGatesMonitorRule>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DeploymentGatesEvaluationRule::DeploymentGatesMonitorRule(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::DeploymentGatesFDDRule>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(DeploymentGatesEvaluationRule::DeploymentGatesFDDRule(_v));
            }
        }

        return Ok(DeploymentGatesEvaluationRule::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
