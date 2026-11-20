// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SplitGroupBackendDefinition20270101SourceWidgetDefinition {
	BarChartBackendDefinition(Box<crate::datadogV1::model::BarChartBackendDefinition>),
	ChangeBackendDefinition(Box<crate::datadogV1::model::ChangeBackendDefinition>),
	DistributionBackendDefinition(Box<crate::datadogV1::model::DistributionBackendDefinition>),
	GeomapBackendDefinition(Box<crate::datadogV1::model::GeomapBackendDefinition>),
	HeatgridDefinition(Box<crate::datadogV1::model::HeatgridDefinition>),
	QueryTableBackendDefinition(Box<crate::datadogV1::model::QueryTableBackendDefinition>),
	QueryValueBackendDefinition(Box<crate::datadogV1::model::QueryValueBackendDefinition>),
	ScatterplotBackendDefinition(Box<crate::datadogV1::model::ScatterplotBackendDefinition>),
	SunburstBackendDefinition(Box<crate::datadogV1::model::SunburstBackendDefinition>),
	TimeseriesWidgetDefinition(Box<crate::datadogV1::model::TimeseriesWidgetDefinition>),
	ToplistBackendDefinition(Box<crate::datadogV1::model::ToplistBackendDefinition>),
	TreemapBackendDefinition(Box<crate::datadogV1::model::TreemapBackendDefinition>),
	WildcardBackendDefinition(Box<crate::datadogV1::model::WildcardBackendDefinition>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SplitGroupBackendDefinition20270101SourceWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::BarChartBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::BarChartBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ChangeBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::ChangeBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::DistributionBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::DistributionBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GeomapBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::GeomapBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::HeatgridDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::HeatgridDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::QueryTableBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::QueryTableBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::QueryValueBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::QueryValueBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ScatterplotBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::ScatterplotBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SunburstBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::SunburstBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TimeseriesWidgetDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::TimeseriesWidgetDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ToplistBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::ToplistBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TreemapBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::TreemapBackendDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::WildcardBackendDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::WildcardBackendDefinition(_v));
            }
        }
	
        return Ok(SplitGroupBackendDefinition20270101SourceWidgetDefinition::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
