// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TextMatch {
	TextMatch20270101OneOf(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf>),
	TextMatch20270101OneOf4462865408(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462865408>),
	TextMatch20270101OneOf4462866048(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462866048>),
	TextMatch20270101OneOf4462866688(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462866688>),
	TextMatch20270101OneOf4462867328(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462867328>),
	TextMatch20270101OneOf4462867968(Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462867968>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TextMatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462865408>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf4462865408(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462866048>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf4462866048(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462866688>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf4462866688(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462867328>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf4462867328(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TextMatch20270101OneOf4462867968>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextMatch::TextMatch20270101OneOf4462867968(_v));
            }
        }
	
        return Ok(TextMatch::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
