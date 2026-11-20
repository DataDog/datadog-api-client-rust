// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Hour {
	Hour12Format(Box<crate::datadogV1_20270101::model::Hour12Format>),
	Hour24Format(Box<crate::datadogV1_20270101::model::Hour24Format>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for Hour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::Hour12Format>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::Hour12Format::UnparsedObject(_v) => {},
				_ => {
					return Ok(Hour::Hour12Format(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::Hour24Format>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::Hour24Format::UnparsedObject(_v) => {},
				_ => {
					return Ok(Hour::Hour24Format(_v))
				}
			}
		}
	
        return Ok(Hour::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
