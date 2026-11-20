// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RetentionSearchCohortCriteriaTimeInterval20270101Value {
	CalendarIntervalDay(Box<crate::datadogV1::model::CalendarIntervalDay>),
	CalendarIntervalWeek(Box<crate::datadogV1::model::CalendarIntervalWeek>),
	CalendarIntervalMonth(Box<crate::datadogV1::model::CalendarIntervalMonth>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for RetentionSearchCohortCriteriaTimeInterval20270101Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::CalendarIntervalDay>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RetentionSearchCohortCriteriaTimeInterval20270101Value::CalendarIntervalDay(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::CalendarIntervalWeek>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RetentionSearchCohortCriteriaTimeInterval20270101Value::CalendarIntervalWeek(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::CalendarIntervalMonth>>(value.clone()) {
			if !_v._unparsed {
                return Ok(RetentionSearchCohortCriteriaTimeInterval20270101Value::CalendarIntervalMonth(_v));
            }
        }
	
        return Ok(RetentionSearchCohortCriteriaTimeInterval20270101Value::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
