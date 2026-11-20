// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HeatmapBackendRequest {
	HeatmapBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf>),
	HeatmapBackendRequest20270101OneOf4458138496(Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458138496>),
	HeatmapBackendRequest20270101OneOf4458139936(Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458139936>),
	HeatmapBackendRequest20270101OneOf4458140576(Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458140576>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HeatmapBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatmapBackendRequest::HeatmapBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458138496>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatmapBackendRequest::HeatmapBackendRequest20270101OneOf4458138496(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458139936>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatmapBackendRequest::HeatmapBackendRequest20270101OneOf4458139936(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatmapBackendRequest20270101OneOf4458140576>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatmapBackendRequest::HeatmapBackendRequest20270101OneOf4458140576(_v));
            }
        }
	
        return Ok(HeatmapBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
