// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GeomapBackendPointsRequest {
	GeomapBackendPointsRequest20270101OneOf(Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf>),
	GeomapBackendPointsRequest20270101OneOf4457225472(Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457225472>),
	GeomapBackendPointsRequest20270101OneOf4457227232(Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457227232>),
	GeomapBackendPointsRequest20270101OneOf4457228992(Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457228992>),
	GeomapBackendPointsRequest20270101OneOf4457230272(Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457230272>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GeomapBackendPointsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendPointsRequest::GeomapBackendPointsRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457225472>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendPointsRequest::GeomapBackendPointsRequest20270101OneOf4457225472(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457227232>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendPointsRequest::GeomapBackendPointsRequest20270101OneOf4457227232(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457228992>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendPointsRequest::GeomapBackendPointsRequest20270101OneOf4457228992(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendPointsRequest20270101OneOf4457230272>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendPointsRequest::GeomapBackendPointsRequest20270101OneOf4457230272(_v));
            }
        }
	
        return Ok(GeomapBackendPointsRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
