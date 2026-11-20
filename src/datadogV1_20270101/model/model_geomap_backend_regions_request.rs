// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GeomapBackendRegionsRequest {
	GeomapBackendRegionsRequest20270101OneOf(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf>),
	GeomapBackendRegionsRequest20270101OneOf4457233792(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457233792>),
	GeomapBackendRegionsRequest20270101OneOf4457448128(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457448128>),
	GeomapBackendRegionsRequest20270101OneOf4457449888(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457449888>),
	GeomapBackendRegionsRequest20270101OneOf4457450848(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457450848>),
	GeomapBackendRegionsRequest20270101OneOf4457451808(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457451808>),
	GeomapBackendRegionsRequest20270101OneOf4457452768(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457452768>),
	GeomapBackendRegionsRequest20270101OneOf4457453728(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457453728>),
	GeomapBackendRegionsRequest20270101OneOf4457454688(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457454688>),
	GeomapBackendRegionsRequest20270101OneOf4457455648(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457455648>),
	GeomapBackendRegionsRequest20270101OneOf4457456608(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457456608>),
	GeomapBackendRegionsRequest20270101OneOf4457457568(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457457568>),
	GeomapBackendRegionsRequest20270101OneOf4457458528(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457458528>),
	GeomapBackendRegionsRequest20270101OneOf4457459488(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457459488>),
	GeomapBackendRegionsRequest20270101OneOf4457460448(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457460448>),
	GeomapBackendRegionsRequest20270101OneOf4457461408(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457461408>),
	GeomapBackendRegionsRequest20270101OneOf4457462368(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457462368>),
	GeomapBackendRegionsRequest20270101OneOf4457463328(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457463328>),
	GeomapBackendRegionsRequest20270101OneOf4457645216(Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457645216>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GeomapBackendRegionsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457233792>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457233792(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457448128>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457448128(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457449888>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457449888(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457450848>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457450848(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457451808>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457451808(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457452768>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457452768(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457453728>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457453728(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457454688>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457454688(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457455648>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457455648(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457456608>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457456608(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457457568>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457457568(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457458528>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457458528(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457459488>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457459488(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457460448>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457460448(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457461408>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457461408(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457462368>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457462368(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457463328>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457463328(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendRegionsRequest20270101OneOf4457645216>>(value.clone()) {
			if !_v._unparsed {
                return Ok(GeomapBackendRegionsRequest::GeomapBackendRegionsRequest20270101OneOf4457645216(_v));
            }
        }
	
        return Ok(GeomapBackendRegionsRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
