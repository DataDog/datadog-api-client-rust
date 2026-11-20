// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum QueryTableBackendRequest {
	QueryTableBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf>),
	QueryTableBackendRequest20270101OneOf4459913248(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459913248>),
	QueryTableBackendRequest20270101OneOf4459916288(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459916288>),
	QueryTableBackendRequest20270101OneOf4459919328(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459919328>),
	QueryTableBackendRequest20270101OneOf4460101856(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460101856>),
	QueryTableBackendRequest20270101OneOf4460104096(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460104096>),
	QueryTableBackendRequest20270101OneOf4460106336(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460106336>),
	QueryTableBackendRequest20270101OneOf4460108576(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460108576>),
	QueryTableBackendRequest20270101OneOf4460110816(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460110816>),
	QueryTableBackendRequest20270101OneOf4460113056(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460113056>),
	QueryTableBackendRequest20270101OneOf4460115296(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460115296>),
	QueryTableBackendRequest20270101OneOf4460117536(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460117536>),
	QueryTableBackendRequest20270101OneOf4460300064(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460300064>),
	QueryTableBackendRequest20270101OneOf4460302304(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460302304>),
	QueryTableBackendRequest20270101OneOf4460304544(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460304544>),
	QueryTableBackendRequest20270101OneOf4460306784(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460306784>),
	QueryTableBackendRequest20270101OneOf4460309024(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460309024>),
	QueryTableBackendRequest20270101OneOf4460311264(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460311264>),
	QueryTableBackendRequest20270101OneOf4460313504(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460313504>),
	QueryTableBackendRequest20270101OneOf4460513216(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460513216>),
	QueryTableBackendRequest20270101OneOf4460516096(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460516096>),
	QueryTableBackendRequest20270101OneOf4460519136(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460519136>),
	QueryTableBackendRequest20270101OneOf4460522016(Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460522016>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for QueryTableBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459913248>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4459913248(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459916288>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4459916288(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4459919328>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4459919328(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460101856>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460101856(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460104096>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460104096(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460106336>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460106336(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460108576>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460108576(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460110816>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460110816(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460113056>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460113056(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460115296>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460115296(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460117536>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460117536(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460300064>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460300064(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460302304>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460302304(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460304544>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460304544(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460306784>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460306784(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460309024>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460309024(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460311264>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460311264(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460313504>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460313504(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460513216>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460513216(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460516096>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460516096(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460519136>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460519136(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendRequest20270101OneOf4460522016>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryTableBackendRequest::QueryTableBackendRequest20270101OneOf4460522016(_v));
            }
        }
	
        return Ok(QueryTableBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
