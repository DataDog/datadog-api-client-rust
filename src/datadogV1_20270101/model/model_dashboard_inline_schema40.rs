// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DashboardInlineSchema40 {
	FormulaDatasetMetricQuery(Box<crate::datadogV1_20270101::model::FormulaDatasetMetricQuery>),
	FormulaDatasetApmMetricsQuery(Box<crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DashboardInlineSchema40 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDatasetMetricQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardInlineSchema40::FormulaDatasetMetricQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDatasetApmMetricsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardInlineSchema40::FormulaDatasetApmMetricsQuery(_v));
            }
        }
	
        return Ok(DashboardInlineSchema40::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
