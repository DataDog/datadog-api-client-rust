// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableColumnFormat {
	NumberBarTableColumnFormatNumber(Box<crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumber>),
	NumberBarTableColumnFormatBar(Box<crate::datadogV1_20270101::model::NumberBarTableColumnFormatBar>),
	TrendTableColumnFormat(Box<crate::datadogV1_20270101::model::TrendTableColumnFormat>),
	StringTableColumnFormat(Box<crate::datadogV1_20270101::model::StringTableColumnFormat>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TableColumnFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::NumberBarTableColumnFormatNumber>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnFormat::NumberBarTableColumnFormatNumber(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::NumberBarTableColumnFormatBar>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnFormat::NumberBarTableColumnFormatBar(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TrendTableColumnFormat>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnFormat::TrendTableColumnFormat(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::StringTableColumnFormat>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnFormat::StringTableColumnFormat(_v));
            }
        }
	
        return Ok(TableColumnFormat::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
