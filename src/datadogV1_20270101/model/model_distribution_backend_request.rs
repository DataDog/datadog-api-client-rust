// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DistributionBackendRequest {
	DistributionBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf>),
	DistributionBackendRequest20270101OneOf4455761856(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455761856>),
	DistributionBackendRequest20270101OneOf4455763296(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455763296>),
	DistributionBackendRequest20270101OneOf4455763936(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455763936>),
	DistributionBackendRequest20270101OneOf4455764576(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455764576>),
	DistributionBackendRequest20270101OneOf4455765216(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455765216>),
	DistributionBackendRequest20270101OneOf4455765856(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455765856>),
	DistributionBackendRequest20270101OneOf4455766496(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455766496>),
	DistributionBackendRequest20270101OneOf4455767136(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455767136>),
	DistributionBackendRequest20270101OneOf4455767776(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455767776>),
	DistributionBackendRequest20270101OneOf4455768416(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455768416>),
	DistributionBackendRequest20270101OneOf4455769056(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455769056>),
	DistributionBackendRequest20270101OneOf4455769696(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455769696>),
	DistributionBackendRequest20270101OneOf4455770336(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455770336>),
	DistributionBackendRequest20270101OneOf4455770976(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455770976>),
	DistributionBackendRequest20270101OneOf4455771616(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455771616>),
	DistributionBackendRequest20270101OneOf4455772256(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455772256>),
	DistributionBackendRequest20270101OneOf4455772896(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455772896>),
	DistributionBackendRequest20270101OneOf4455773536(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455773536>),
	DistributionBackendRequest20270101OneOf4455774496(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455774496>),
	DistributionBackendRequest20270101OneOf4455775456(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455775456>),
	DistributionBackendRequest20270101OneOf4455989472(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455989472>),
	DistributionBackendRequest20270101OneOf4455990432(Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455990432>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DistributionBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455761856>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455761856(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455763296>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455763296(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455763936>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455763936(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455764576>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455764576(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455765216>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455765216(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455765856>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455765856(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455766496>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455766496(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455767136>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455767136(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455767776>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455767776(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455768416>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455768416(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455769056>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455769056(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455769696>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455769696(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455770336>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455770336(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455770976>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455770976(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455771616>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455771616(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455772256>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455772256(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455772896>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455772896(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455773536>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455773536(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455774496>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455774496(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455775456>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455775456(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455989472>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455989472(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendRequest20270101OneOf4455990432>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DistributionBackendRequest::DistributionBackendRequest20270101OneOf4455990432(_v));
            }
        }
	
        return Ok(DistributionBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
