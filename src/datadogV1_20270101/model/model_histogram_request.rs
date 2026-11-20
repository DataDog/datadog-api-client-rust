// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HistogramRequest {
	MetricsHistogramRequest(Box<crate::datadogV1_20270101::model::MetricsHistogramRequest>),
	ApmStatsHistogramRequest(Box<crate::datadogV1_20270101::model::ApmStatsHistogramRequest>),
	ApmMetricsHistogramRequest(Box<crate::datadogV1_20270101::model::ApmMetricsHistogramRequest>),
	EventsHistogramRequest(Box<crate::datadogV1_20270101::model::EventsHistogramRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HistogramRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MetricsHistogramRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HistogramRequest::MetricsHistogramRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ApmStatsHistogramRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HistogramRequest::ApmStatsHistogramRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ApmMetricsHistogramRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HistogramRequest::ApmMetricsHistogramRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventsHistogramRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(HistogramRequest::EventsHistogramRequest(_v));
            }
        }
	
        return Ok(HistogramRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
