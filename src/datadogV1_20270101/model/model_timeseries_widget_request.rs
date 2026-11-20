// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TimeseriesWidgetRequest {
	TimeseriesWidgetFormulaRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequest>),
	TimeseriesWidgetMetricRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetMetricRequest>),
	TimeseriesWidgetLogRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetLogRequest>),
	TimeseriesWidgetApmRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetApmRequest>),
	TimeseriesWidgetNetworkRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetNetworkRequest>),
	TimeseriesWidgetRumRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetRumRequest>),
	TimeseriesWidgetSecurityRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetSecurityRequest>),
	TimeseriesWidgetComplianceFindingsRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetComplianceFindingsRequest>),
	TimeseriesWidgetEventRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetEventRequest>),
	TimeseriesWidgetProfilingMetricsRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetProfilingMetricsRequest>),
	TimeseriesWidgetCiPipelineRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetCiPipelineRequest>),
	TimeseriesWidgetCiTestRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetCiTestRequest>),
	TimeseriesWidgetAuditRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetAuditRequest>),
	TimeseriesWidgetAppsecSpanRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetAppsecSpanRequest>),
	TimeseriesWidgetSyntheticsBatchesRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetSyntheticsBatchesRequest>),
	TimeseriesWidgetSyntheticsTestRunsRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetSyntheticsTestRunsRequest>),
	TimeseriesWidgetProcessRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetProcessRequest>),
	TimeseriesWidgetLegacyProfileMetricsRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetLegacyProfileMetricsRequest>),
	TimeseriesWidgetLegacyEventRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetLegacyEventRequest>),
	TimeseriesWidgetDdsqlRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetDdsqlRequest>),
	TimeseriesWidgetAdvancedQueryRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetAdvancedQueryRequest>),
	TimeseriesWidgetLocalDatasetRequest(Box<crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TimeseriesWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetFormulaRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetFormulaRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetMetricRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetMetricRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetLogRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetLogRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetApmRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetApmRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetNetworkRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetNetworkRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetRumRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetRumRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetSecurityRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetSecurityRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetComplianceFindingsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetComplianceFindingsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetEventRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetEventRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetProfilingMetricsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetProfilingMetricsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetCiPipelineRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetCiPipelineRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetCiTestRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetCiTestRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetAuditRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetAuditRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetAppsecSpanRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetAppsecSpanRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetSyntheticsBatchesRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetSyntheticsBatchesRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetSyntheticsTestRunsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetSyntheticsTestRunsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetProcessRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetProcessRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetLegacyProfileMetricsRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetLegacyProfileMetricsRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetLegacyEventRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetLegacyEventRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetDdsqlRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetDdsqlRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetAdvancedQueryRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetAdvancedQueryRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesWidgetLocalDatasetRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TimeseriesWidgetRequest::TimeseriesWidgetLocalDatasetRequest(_v));
            }
        }
	
        return Ok(TimeseriesWidgetRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
