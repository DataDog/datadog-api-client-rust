// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Remediation details for the deployment. Optional, but required to calculate failed deployment recovery time. Specify either `id` or `version` to identify the remediation deployment, but not both.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DORADeploymentPatchByVersionRemediation {
    DORADeploymentPatchByVersionRemediationByID(
        Box<crate::datadogV2::model::DORADeploymentPatchByVersionRemediationByID>,
    ),
    DORADeploymentPatchByVersionRemediationByVersion(
        Box<crate::datadogV2::model::DORADeploymentPatchByVersionRemediationByVersion>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DORADeploymentPatchByVersionRemediation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DORADeploymentPatchByVersionRemediationByID>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DORADeploymentPatchByVersionRemediation::DORADeploymentPatchByVersionRemediationByID(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DORADeploymentPatchByVersionRemediationByVersion>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DORADeploymentPatchByVersionRemediation::DORADeploymentPatchByVersionRemediationByVersion(_v));
            }
        }

        return Ok(DORADeploymentPatchByVersionRemediation::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
