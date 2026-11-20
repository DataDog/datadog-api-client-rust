// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TabularProjection {
	ScatterplotProjection(Box<crate::datadogV1_20270101::model::ScatterplotProjection>),
	TreeDataProjectionBarChart(Box<crate::datadogV1_20270101::model::TreeDataProjectionBarChart>),
	TreeDataProjectionSunburst(Box<crate::datadogV1_20270101::model::TreeDataProjectionSunburst>),
	TreeDataProjectionToplist(Box<crate::datadogV1_20270101::model::TreeDataProjectionToplist>),
	TreeDataProjectionTreemap(Box<crate::datadogV1_20270101::model::TreeDataProjectionTreemap>),
	QueryValueProjection(Box<crate::datadogV1_20270101::model::QueryValueProjection>),
	QueryTableProjection(Box<crate::datadogV1_20270101::model::QueryTableProjection>),
	RunChartProjection(Box<crate::datadogV1_20270101::model::RunChartProjection>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TabularProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotProjection>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::ScatterplotProjection(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TreeDataProjectionBarChart>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::TreeDataProjectionBarChart(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TreeDataProjectionSunburst>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::TreeDataProjectionSunburst(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TreeDataProjectionToplist>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::TreeDataProjectionToplist(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TreeDataProjectionTreemap>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::TreeDataProjectionTreemap(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueProjection>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::QueryValueProjection(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableProjection>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::QueryTableProjection(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RunChartProjection>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TabularProjection::RunChartProjection(_v));
            }
        }
	
        return Ok(TabularProjection::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
