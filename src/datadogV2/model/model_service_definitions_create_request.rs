// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Create service definitions request.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ServiceDefinitionsCreateRequest {
    ServiceDefinitionV2Dot2(Box<crate::datadogV2::model::ServiceDefinitionV2Dot2>),
    ServiceDefinitionV2Dot1(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1>),
    ServiceDefinitionV2(Box<crate::datadogV2::model::ServiceDefinitionV2>),
    ServiceDefinitionRaw(String),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ServiceDefinitionsCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Dot2>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionsCreateRequest::ServiceDefinitionV2Dot2(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceDefinitionV2Dot1>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ServiceDefinitionsCreateRequest::ServiceDefinitionV2Dot1(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ServiceDefinitionV2>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ServiceDefinitionsCreateRequest::ServiceDefinitionV2(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(ServiceDefinitionsCreateRequest::ServiceDefinitionRaw(_v));
        }

        return Ok(ServiceDefinitionsCreateRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
