// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SloMeasure {
	DashboardInlineSchema46(Box<crate::datadogV1_20270101::model::DashboardInlineSchema46>),
	SloMeasureEvents(Box<crate::datadogV1_20270101::model::SloMeasureEvents>),
	SloMeasureMinutes(Box<crate::datadogV1_20270101::model::SloMeasureMinutes>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SloMeasure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DashboardInlineSchema46>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::DashboardInlineSchema46::UnparsedObject(_v) => {},
				_ => {
					return Ok(SloMeasure::DashboardInlineSchema46(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SloMeasureEvents>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::SloMeasureEvents::UnparsedObject(_v) => {},
				_ => {
					return Ok(SloMeasure::SloMeasureEvents(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SloMeasureMinutes>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::SloMeasureMinutes::UnparsedObject(_v) => {},
				_ => {
					return Ok(SloMeasure::SloMeasureMinutes(_v))
				}
			}
		}
	
        return Ok(SloMeasure::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
