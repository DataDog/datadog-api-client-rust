// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AllListRequestVariants {
	ListStreamRequest(Box<crate::datadogV1_20270101::model::ListStreamRequest>),
	CompositeStreamRequest(Box<crate::datadogV1_20270101::model::CompositeStreamRequest>),
	DatasetListRequest(Box<crate::datadogV1_20270101::model::DatasetListRequest>),
	InfrastructureListRequest(Box<crate::datadogV1_20270101::model::InfrastructureListRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AllListRequestVariants {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ListStreamRequest>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::ListStreamRequest::UnparsedObject(_v) => {},
				_ => {
					return Ok(AllListRequestVariants::ListStreamRequest(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CompositeStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AllListRequestVariants::CompositeStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DatasetListRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AllListRequestVariants::DatasetListRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::InfrastructureListRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AllListRequestVariants::InfrastructureListRequest(_v));
            }
        }
	
        return Ok(AllListRequestVariants::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
