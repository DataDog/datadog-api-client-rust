// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ListStreamRequest {
	EventsListStreamRequest(Box<crate::datadogV1_20270101::model::EventsListStreamRequest>),
	IssueStreamRequest(Box<crate::datadogV1_20270101::model::IssueStreamRequest>),
	CaseStreamRequest(Box<crate::datadogV1_20270101::model::CaseStreamRequest>),
	WatchdogAlertsRequest(Box<crate::datadogV1_20270101::model::WatchdogAlertsRequest>),
	IncidentsStreamRequest(Box<crate::datadogV1_20270101::model::IncidentsStreamRequest>),
	DatabaseRecommendationsStreamRequest(Box<crate::datadogV1_20270101::model::DatabaseRecommendationsStreamRequest>),
	ApmRecommendationsStreamRequest(Box<crate::datadogV1_20270101::model::ApmRecommendationsStreamRequest>),
	DetectionRuleStreamRequest(Box<crate::datadogV1_20270101::model::DetectionRuleStreamRequest>),
	DataObservabilityAssetsRequest(Box<crate::datadogV1_20270101::model::DataObservabilityAssetsRequest>),
	DoraListStreamRequest(Box<crate::datadogV1_20270101::model::DoraListStreamRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ListStreamRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventsListStreamRequest>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::EventsListStreamRequest::UnparsedObject(_v) => {},
				_ => {
					return Ok(ListStreamRequest::EventsListStreamRequest(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::IssueStreamRequest>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::IssueStreamRequest::UnparsedObject(_v) => {},
				_ => {
					return Ok(ListStreamRequest::IssueStreamRequest(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CaseStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::CaseStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WatchdogAlertsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::WatchdogAlertsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::IncidentsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::IncidentsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DatabaseRecommendationsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::DatabaseRecommendationsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ApmRecommendationsStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::ApmRecommendationsStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DetectionRuleStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::DetectionRuleStreamRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DataObservabilityAssetsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::DataObservabilityAssetsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DoraListStreamRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ListStreamRequest::DoraListStreamRequest(_v));
            }
        }
	
        return Ok(ListStreamRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
