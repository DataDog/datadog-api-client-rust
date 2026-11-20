// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SunburstKnownLegend {
	SunburstKnownLegend20270101OneOf(Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf>),
	SunburstKnownLegend20270101OneOf4462427680(Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf4462427680>),
	SunburstKnownLegend20270101OneOf4462657920(Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf4462657920>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SunburstKnownLegend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SunburstKnownLegend::SunburstKnownLegend20270101OneOf(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf4462427680>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SunburstKnownLegend::SunburstKnownLegend20270101OneOf4462427680(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SunburstKnownLegend20270101OneOf4462657920>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SunburstKnownLegend::SunburstKnownLegend20270101OneOf4462657920(_v));
            }
        }
	
        return Ok(SunburstKnownLegend::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
