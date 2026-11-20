// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableFormula {
	TableFormula20270101OneOf(Box<crate::datadogV1_20270101::model::TableFormula20270101OneOf>),
	TableFormula20270101OneOf4462669760(Box<crate::datadogV1_20270101::model::TableFormula20270101OneOf4462669760>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TableFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableFormula20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableFormula::TableFormula20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableFormula20270101OneOf4462669760>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableFormula::TableFormula20270101OneOf4462669760(_v));
            }
        }
	
        return Ok(TableFormula::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
