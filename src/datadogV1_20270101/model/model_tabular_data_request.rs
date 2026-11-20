// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TabularDataRequest {
	AllListRequestVariants(Box<crate::datadogV1_20270101::model::AllListRequestVariants>),
	TabularAdvancedQueryRequest(Box<crate::datadogV1_20270101::model::TabularAdvancedQueryRequest>),
	TimeseriesFormulaRequest(Box<crate::datadogV1_20270101::model::TimeseriesFormulaRequest>),
	ScalarFormulaRequest(Box<crate::datadogV1_20270101::model::ScalarFormulaRequest>),
	TabularLocalDatasetRequest(Box<crate::datadogV1_20270101::model::TabularLocalDatasetRequest>),
	HistogramRequest(Box<crate::datadogV1_20270101::model::HistogramRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TabularDataRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AllListRequestVariants>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::AllListRequestVariants::UnparsedObject(_v) => {},
				_ => {
					return Ok(TabularDataRequest::AllListRequestVariants(_v))
				}
			}
		}
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TabularAdvancedQueryRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularDataRequest::TabularAdvancedQueryRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesFormulaRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularDataRequest::TimeseriesFormulaRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScalarFormulaRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularDataRequest::ScalarFormulaRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TabularLocalDatasetRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularDataRequest::TabularLocalDatasetRequest(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HistogramRequest>>(value.clone()) {
			match *_v {
				crate::datadogV1_20270101::model::HistogramRequest::UnparsedObject(_v) => {},
				_ => {
					return Ok(TabularDataRequest::HistogramRequest(_v))
				}
			}
		}
	
        return Ok(TabularDataRequest::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
