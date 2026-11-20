// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BarChartBackendRequest {
	BarChartBackendRequest20270101OneOf(Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf>),
	BarChartBackendRequest20270101OneOf4454434112(Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454434112>),
	BarChartBackendRequest20270101OneOf4454435552(Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454435552>),
	BarChartBackendRequest20270101OneOf4454436672(Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454436672>),
	BarChartBackendRequest20270101OneOf4454438272(Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454438272>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for BarChartBackendRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendRequest::BarChartBackendRequest20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454434112>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendRequest::BarChartBackendRequest20270101OneOf4454434112(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454435552>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendRequest::BarChartBackendRequest20270101OneOf4454435552(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454436672>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendRequest::BarChartBackendRequest20270101OneOf4454436672(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendRequest20270101OneOf4454438272>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendRequest::BarChartBackendRequest20270101OneOf4454438272(_v));
            }
        }
	
        return Ok(BarChartBackendRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
