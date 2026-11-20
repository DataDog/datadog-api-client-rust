// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ToplistBackendRequest {
	ToplistBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf>),
	ToplistBackendRequest20270101OneOf4463319360(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463319360>),
	ToplistBackendRequest20270101OneOf4463320800(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463320800>),
	ToplistBackendRequest20270101OneOf4463321920(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463321920>),
	ToplistBackendRequest20270101OneOf4463323520(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463323520>),
	ToplistBackendRequest20270101OneOf4463325120(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463325120>),
	ToplistBackendRequest20270101OneOf4463325920(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463325920>),
	ToplistBackendRequest20270101OneOf4463326720(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463326720>),
	ToplistBackendRequest20270101OneOf4463327520(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463327520>),
	ToplistBackendRequest20270101OneOf4463328320(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463328320>),
	ToplistBackendRequest20270101OneOf4463329120(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463329120>),
	ToplistBackendRequest20270101OneOf4463526592(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463526592>),
	ToplistBackendRequest20270101OneOf4463527392(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463527392>),
	ToplistBackendRequest20270101OneOf4463528192(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463528192>),
	ToplistBackendRequest20270101OneOf4463528992(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463528992>),
	ToplistBackendRequest20270101OneOf4463529792(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463529792>),
	ToplistBackendRequest20270101OneOf4463530592(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463530592>),
	ToplistBackendRequest20270101OneOf4463531392(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463531392>),
	ToplistBackendRequest20270101OneOf4463532192(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463532192>),
	ToplistBackendRequest20270101OneOf4463532992(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463532992>),
	ToplistBackendRequest20270101OneOf4463533792(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463533792>),
	ToplistBackendRequest20270101OneOf4463535232(Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463535232>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ToplistBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463319360>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463319360(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463320800>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463320800(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463321920>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463321920(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463323520>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463323520(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463325120>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463325120(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463325920>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463325920(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463326720>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463326720(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463327520>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463327520(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463328320>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463328320(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463329120>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463329120(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463526592>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463526592(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463527392>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463527392(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463528192>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463528192(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463528992>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463528992(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463529792>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463529792(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463530592>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463530592(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463531392>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463531392(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463532192>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463532192(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463532992>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463532992(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463533792>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463533792(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendRequest20270101OneOf4463535232>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ToplistBackendRequest::ToplistBackendRequest20270101OneOf4463535232(_v));
            }
        }
	
        return Ok(ToplistBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
