// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BarChartBackendDefinitionStyle20270101Display {
	BarChartBackendDefinitionStyle20270101DisplayOneOf(Box<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101DisplayOneOf>),
	BarChartBackendDefinitionStyle20270101DisplayOneOf4453561856(Box<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101DisplayOneOf4453561856>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for BarChartBackendDefinitionStyle20270101Display {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101DisplayOneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendDefinitionStyle20270101Display::BarChartBackendDefinitionStyle20270101DisplayOneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendDefinitionStyle20270101DisplayOneOf4453561856>>(value.clone()) {
			if !_v._unparsed {
                return Ok(BarChartBackendDefinitionStyle20270101Display::BarChartBackendDefinitionStyle20270101DisplayOneOf4453561856(_v));
            }
        }
	
        return Ok(BarChartBackendDefinitionStyle20270101Display::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
