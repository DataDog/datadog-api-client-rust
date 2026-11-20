// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HostmapBackendDefinition20270101Requests {
	MetricHostmapRequests(Box<crate::datadogV1::model::MetricHostmapRequests>),
	HostmapInfrastructureRequest(Box<crate::datadogV1::model::HostmapInfrastructureRequest>),
	HostmapDataProjectionRequest(Box<crate::datadogV1::model::HostmapDataProjectionRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HostmapBackendDefinition20270101Requests {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::MetricHostmapRequests>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HostmapBackendDefinition20270101Requests::MetricHostmapRequests(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::HostmapInfrastructureRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HostmapBackendDefinition20270101Requests::HostmapInfrastructureRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::HostmapDataProjectionRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HostmapBackendDefinition20270101Requests::HostmapDataProjectionRequest(_v));
            }
        }
	
        return Ok(HostmapBackendDefinition20270101Requests::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
