// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScatterPlotLegendType {
	ScatterPlotLegendType20270101OneOf(Box<crate::datadogV1_20270101::model::ScatterPlotLegendType20270101OneOf>),
	ScatterPlotLegendType20270101OneOf4461516160(Box<crate::datadogV1_20270101::model::ScatterPlotLegendType20270101OneOf4461516160>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScatterPlotLegendType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterPlotLegendType20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterPlotLegendType::ScatterPlotLegendType20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterPlotLegendType20270101OneOf4461516160>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ScatterPlotLegendType::ScatterPlotLegendType20270101OneOf4461516160(_v));
            }
        }
	
        return Ok(ScatterPlotLegendType::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
