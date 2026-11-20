// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RunChartProjectionDimension {
	RunChartXDimension(Box<crate::datadogV1_20270101::model::RunChartXDimension>),
	RunChartValueDimension(Box<crate::datadogV1_20270101::model::RunChartValueDimension>),
	RunChartGroupDimension(Box<crate::datadogV1_20270101::model::RunChartGroupDimension>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for RunChartProjectionDimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RunChartXDimension>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RunChartProjectionDimension::RunChartXDimension(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RunChartValueDimension>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RunChartProjectionDimension::RunChartValueDimension(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RunChartGroupDimension>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RunChartProjectionDimension::RunChartGroupDimension(_v));
            }
        }
	
        return Ok(RunChartProjectionDimension::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
