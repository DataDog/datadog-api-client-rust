// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The `ServiceNowBasicAuthUpdate` `instance`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ServiceNowBasicAuthUpdateInstance {
    ActionConnectionLiteralValue(String),
    ActionConnectionAWSSecretsManagerReference(
        Box<crate::datadogV2::model::ActionConnectionAWSSecretsManagerReference>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ServiceNowBasicAuthUpdateInstance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ServiceNowBasicAuthUpdateInstance::ActionConnectionLiteralValue(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ActionConnectionAWSSecretsManagerReference>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ServiceNowBasicAuthUpdateInstance::ActionConnectionAWSSecretsManagerReference(
                        _v,
                    ),
                );
            }
        }

        return Ok(ServiceNowBasicAuthUpdateInstance::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
