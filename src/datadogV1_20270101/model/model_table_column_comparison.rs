// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableColumnComparison {
	TableColumnComparisonWithSelf(Box<crate::datadogV1_20270101::model::TableColumnComparisonWithSelf>),
	TableColumnComparisonWithOtherColumnDiffFrom(Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnDiffFrom>),
	TableColumnComparisonWithOtherColumnPercentOf(Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnPercentOf>),
	TableColumnComparisonWithOtherColumnPercentDiffFrom(Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnPercentDiffFrom>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TableColumnComparison {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableColumnComparisonWithSelf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnComparison::TableColumnComparisonWithSelf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnDiffFrom>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnComparison::TableColumnComparisonWithOtherColumnDiffFrom(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnPercentOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnComparison::TableColumnComparisonWithOtherColumnPercentOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TableColumnComparisonWithOtherColumnPercentDiffFrom>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumnComparison::TableColumnComparisonWithOtherColumnPercentDiffFrom(_v));
            }
        }
	
        return Ok(TableColumnComparison::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
