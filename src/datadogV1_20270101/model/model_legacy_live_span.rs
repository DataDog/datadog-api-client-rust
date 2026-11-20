// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LegacyLiveSpan {
	TimeSpan(Box<crate::datadogV1_20270101::model::TimeSpan>),
	DashboardInlineSchema43(Box<crate::datadogV1_20270101::model::DashboardInlineSchema43>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LegacyLiveSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeSpan>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::TimeSpan::UnparsedObject(_v) => {},
				_ => {
					return Ok(LegacyLiveSpan::TimeSpan(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DashboardInlineSchema43>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::DashboardInlineSchema43::UnparsedObject(_v) => {},
				_ => {
					return Ok(LegacyLiveSpan::DashboardInlineSchema43(_v))
				}
			}
		}
	
        return Ok(LegacyLiveSpan::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
