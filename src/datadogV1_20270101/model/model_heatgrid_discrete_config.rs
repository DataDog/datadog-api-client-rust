// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HeatgridDiscreteConfig {
	HeatgridDiscreteConfig20270101OneOf(Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig20270101OneOf>),
	HeatgridDiscreteConfig20270101OneOf4457881312(Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig20270101OneOf4457881312>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HeatgridDiscreteConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatgridDiscreteConfig::HeatgridDiscreteConfig20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig20270101OneOf4457881312>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HeatgridDiscreteConfig::HeatgridDiscreteConfig20270101OneOf4457881312(_v));
            }
        }
	
        return Ok(HeatgridDiscreteConfig::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
