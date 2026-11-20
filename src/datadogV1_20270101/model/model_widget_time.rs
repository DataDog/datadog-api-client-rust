// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WidgetTime {
	WidgetTime20270101OneOf(Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf>),
	WidgetTime20270101OneOf4464103552(Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464103552>),
	WidgetTime20270101OneOf4464104512(Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464104512>),
	WidgetTime20270101OneOf4464105312(Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464105312>),
	WidgetTime20270101OneOf4464105952(Box<crate::datadogV1::model::WidgetTime20270101OneOf4464105952>),
	EmptyObject(Box<crate::datadogV1::model::EmptyObject>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WidgetTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::WidgetTime20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464103552>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::WidgetTime20270101OneOf4464103552(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464104512>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::WidgetTime20270101OneOf4464104512(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WidgetTime20270101OneOf4464105312>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::WidgetTime20270101OneOf4464105312(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::WidgetTime20270101OneOf4464105952>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::WidgetTime20270101OneOf4464105952(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::EmptyObject>>(value.clone()) {
			if !_v._unparsed {
                return Ok(WidgetTime::EmptyObject(_v));
            }
        }
	
        return Ok(WidgetTime::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
