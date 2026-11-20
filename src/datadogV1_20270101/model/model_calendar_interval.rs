// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CalendarInterval {
	CalendarIntervalDay(Box<crate::datadogV1_20270101::model::CalendarIntervalDay>),
	CalendarIntervalWeek(Box<crate::datadogV1_20270101::model::CalendarIntervalWeek>),
	CalendarIntervalMonth(Box<crate::datadogV1_20270101::model::CalendarIntervalMonth>),
	CalendarIntervalYear(Box<crate::datadogV1_20270101::model::CalendarIntervalYear>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CalendarInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CalendarIntervalDay>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CalendarInterval::CalendarIntervalDay(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CalendarIntervalWeek>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CalendarInterval::CalendarIntervalWeek(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CalendarIntervalMonth>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CalendarInterval::CalendarIntervalMonth(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CalendarIntervalYear>>(value.clone()) {
			if !_v._unparsed {
                return Ok(CalendarInterval::CalendarIntervalYear(_v));
            }
        }
	
        return Ok(CalendarInterval::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
