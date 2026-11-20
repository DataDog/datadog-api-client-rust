// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScatterplotBackendTableRequest {
	ScatterplotBackendTableRequest20270101OneOf(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf>),
	ScatterplotBackendTableRequest20270101OneOf4461521920(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461521920>),
	ScatterplotBackendTableRequest20270101OneOf4461523200(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461523200>),
	ScatterplotBackendTableRequest20270101OneOf4461524480(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461524480>),
	ScatterplotBackendTableRequest20270101OneOf4461525600(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461525600>),
	ScatterplotBackendTableRequest20270101OneOf4461756480(Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461756480>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScatterplotBackendTableRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461521920>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf4461521920(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461523200>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf4461523200(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461524480>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf4461524480(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461525600>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf4461525600(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest20270101OneOf4461756480>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterplotBackendTableRequest::ScatterplotBackendTableRequest20270101OneOf4461756480(_v));
            }
        }
	
        return Ok(ScatterplotBackendTableRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
