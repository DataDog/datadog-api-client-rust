// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableColumn {
	ComputeTableColumn(Box<crate::datadogV1_20270101::model::ComputeTableColumn>),
	PresetPeriodTableColumn(Box<crate::datadogV1_20270101::model::PresetPeriodTableColumn>),
	FormulaTableColumn(Box<crate::datadogV1_20270101::model::FormulaTableColumn>),
	ReferenceTableColumn(Box<crate::datadogV1_20270101::model::ReferenceTableColumn>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TableColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ComputeTableColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumn::ComputeTableColumn(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::PresetPeriodTableColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumn::PresetPeriodTableColumn(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaTableColumn>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TableColumn::FormulaTableColumn(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReferenceTableColumn>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::ReferenceTableColumn::UnparsedObject(_v) => {},
				_ => {
					return Ok(TableColumn::ReferenceTableColumn(_v))
				}
			}
		}
	
        return Ok(TableColumn::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
