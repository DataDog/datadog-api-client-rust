// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum QueryValueBackendRequest {
	QueryValueBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf>),
	QueryValueBackendRequest20270101OneOf4460683296(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460683296>),
	QueryValueBackendRequest20270101OneOf4460685056(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460685056>),
	QueryValueBackendRequest20270101OneOf4460686496(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460686496>),
	QueryValueBackendRequest20270101OneOf4460688416(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460688416>),
	QueryValueBackendRequest20270101OneOf4460689536(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460689536>),
	QueryValueBackendRequest20270101OneOf4460690656(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460690656>),
	QueryValueBackendRequest20270101OneOf4460904832(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460904832>),
	QueryValueBackendRequest20270101OneOf4460905952(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460905952>),
	QueryValueBackendRequest20270101OneOf4460907072(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460907072>),
	QueryValueBackendRequest20270101OneOf4460908192(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460908192>),
	QueryValueBackendRequest20270101OneOf4460909312(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460909312>),
	QueryValueBackendRequest20270101OneOf4460910432(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460910432>),
	QueryValueBackendRequest20270101OneOf4460911552(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460911552>),
	QueryValueBackendRequest20270101OneOf4460912672(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460912672>),
	QueryValueBackendRequest20270101OneOf4460913792(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460913792>),
	QueryValueBackendRequest20270101OneOf4460914912(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460914912>),
	QueryValueBackendRequest20270101OneOf4460916032(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460916032>),
	QueryValueBackendRequest20270101OneOf4460917152(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460917152>),
	QueryValueBackendRequest20270101OneOf4460918912(Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460918912>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for QueryValueBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460683296>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460683296(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460685056>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460685056(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460686496>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460686496(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460688416>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460688416(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460689536>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460689536(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460690656>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460690656(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460904832>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460904832(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460905952>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460905952(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460907072>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460907072(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460908192>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460908192(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460909312>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460909312(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460910432>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460910432(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460911552>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460911552(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460912672>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460912672(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460913792>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460913792(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460914912>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460914912(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460916032>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460916032(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460917152>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460917152(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendRequest20270101OneOf4460918912>>(value.clone()) {
			if !_v._unparsed {
                return Ok(QueryValueBackendRequest::QueryValueBackendRequest20270101OneOf4460918912(_v));
            }
        }
	
        return Ok(QueryValueBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
