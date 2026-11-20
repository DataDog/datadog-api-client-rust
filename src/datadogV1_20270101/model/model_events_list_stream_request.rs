// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EventsListStreamRequest {
	EventPlatformStreamRequest(Box<crate::datadogV1_20270101::model::EventPlatformStreamRequest>),
	LogsStreamRequest(Box<crate::datadogV1_20270101::model::LogsStreamRequest>),
	LogsPatternStreamRequest(Box<crate::datadogV1_20270101::model::LogsPatternStreamRequest>),
	LogsTransactionStreamRequest(Box<crate::datadogV1_20270101::model::LogsTransactionStreamRequest>),
	AuditStreamRequest(Box<crate::datadogV1_20270101::model::AuditStreamRequest>),
	TraceStreamRequest(Box<crate::datadogV1_20270101::model::TraceStreamRequest>),
	ErrorsStreamRequest(Box<crate::datadogV1_20270101::model::ErrorsStreamRequest>),
	EventStreamRequest(Box<crate::datadogV1_20270101::model::EventStreamRequest>),
	EventPatternStreamRequest(Box<crate::datadogV1_20270101::model::EventPatternStreamRequest>),
	SecurityRuntimePatternStreamRequest(Box<crate::datadogV1_20270101::model::SecurityRuntimePatternStreamRequest>),
	RumStreamRequest(Box<crate::datadogV1_20270101::model::RumStreamRequest>),
	RecommendationsStreamRequest(Box<crate::datadogV1_20270101::model::RecommendationsStreamRequest>),
	SecurityRuntimeStreamRequest(Box<crate::datadogV1_20270101::model::SecurityRuntimeStreamRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EventsListStreamRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventPlatformStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::EventPlatformStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LogsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::LogsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LogsPatternStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::LogsPatternStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LogsTransactionStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::LogsTransactionStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AuditStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::AuditStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TraceStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::TraceStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ErrorsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::ErrorsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::EventStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventPatternStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::EventPatternStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SecurityRuntimePatternStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::SecurityRuntimePatternStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RumStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::RumStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RecommendationsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::RecommendationsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SecurityRuntimeStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(EventsListStreamRequest::SecurityRuntimeStreamRequest(_v));
            }
        }
	
        return Ok(EventsListStreamRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
