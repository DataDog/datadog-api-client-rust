// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Configuration for the policy.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MonitorConfigPolicyPolicyCreateRequest {
    MonitorConfigPolicyTagPolicyCreateRequest(
        Box<crate::datadogV2::model::MonitorConfigPolicyTagPolicyCreateRequest>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for MonitorConfigPolicyPolicyCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::MonitorConfigPolicyTagPolicyCreateRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MonitorConfigPolicyPolicyCreateRequest::MonitorConfigPolicyTagPolicyCreateRequest(_v));
            }
        }

        return Ok(MonitorConfigPolicyPolicyCreateRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
