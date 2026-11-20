// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HeatgridColorConfig {
	HeatgridGradientConfig(Box<crate::datadogV1_20270101::model::HeatgridGradientConfig>),
	HeatgridDiscreteConfig(Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HeatgridColorConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatgridGradientConfig>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::HeatgridGradientConfig::UnparsedObject(_v) => {},
				_ => {
					return Ok(HeatgridColorConfig::HeatgridGradientConfig(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatgridDiscreteConfig>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::HeatgridDiscreteConfig::UnparsedObject(_v) => {},
				_ => {
					return Ok(HeatgridColorConfig::HeatgridDiscreteConfig(_v))
				}
			}
		}
	
        return Ok(HeatgridColorConfig::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
