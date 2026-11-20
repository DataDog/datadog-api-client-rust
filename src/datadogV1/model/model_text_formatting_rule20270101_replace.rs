// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TextFormattingRule20270101Replace {
	TextFormattingRule20270101ReplaceOneOf(Box<crate::datadogV1::model::TextFormattingRule20270101ReplaceOneOf>),
	TextFormattingRule20270101ReplaceOneOf4462863328(Box<crate::datadogV1::model::TextFormattingRule20270101ReplaceOneOf4462863328>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TextFormattingRule20270101Replace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TextFormattingRule20270101ReplaceOneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextFormattingRule20270101Replace::TextFormattingRule20270101ReplaceOneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TextFormattingRule20270101ReplaceOneOf4462863328>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TextFormattingRule20270101Replace::TextFormattingRule20270101ReplaceOneOf4462863328(_v));
            }
        }
	
        return Ok(TextFormattingRule20270101Replace::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
