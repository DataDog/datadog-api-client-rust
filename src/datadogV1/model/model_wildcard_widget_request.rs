// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Request object for the wildcard widget. Each variant represents a distinct data-fetching pattern: scalar formulas, timeseries formulas, list streams, and histograms.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WildcardWidgetRequest {
    TreeMapWidgetRequest(Box<crate::datadogV1::model::TreeMapWidgetRequest>),
    TimeseriesWidgetRequest(Box<crate::datadogV1::model::TimeseriesWidgetRequest>),
    ListStreamWidgetRequest(Box<crate::datadogV1::model::ListStreamWidgetRequest>),
    DistributionWidgetRequest(Box<crate::datadogV1::model::DistributionWidgetRequest>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WildcardWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TreeMapWidgetRequest>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WildcardWidgetRequest::TreeMapWidgetRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TimeseriesWidgetRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WildcardWidgetRequest::TimeseriesWidgetRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ListStreamWidgetRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WildcardWidgetRequest::ListStreamWidgetRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::DistributionWidgetRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WildcardWidgetRequest::DistributionWidgetRequest(_v));
            }
        }

        return Ok(WildcardWidgetRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
