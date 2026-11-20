// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComputationScope {
	CohortScope(Box<crate::datadogV1_20270101::model::CohortScope>),
	ReturnPeriodScope(Box<crate::datadogV1_20270101::model::ReturnPeriodScope>),
	CellScope(Box<crate::datadogV1_20270101::model::CellScope>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ComputationScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CohortScope>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ComputationScope::CohortScope(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReturnPeriodScope>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ComputationScope::ReturnPeriodScope(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CellScope>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ComputationScope::CellScope(_v));
            }
        }
	
        return Ok(ComputationScope::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
