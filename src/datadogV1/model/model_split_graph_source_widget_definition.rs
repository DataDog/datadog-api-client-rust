// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The original widget we are splitting on.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SplitGraphSourceWidgetDefinition {
    ChangeWidgetDefinition(Box<crate::datadogV1::model::ChangeWidgetDefinition>),
    GeomapWidgetDefinition(Box<crate::datadogV1::model::GeomapWidgetDefinition>),
    QueryValueWidgetDefinition(Box<crate::datadogV1::model::QueryValueWidgetDefinition>),
    ScatterPlotWidgetDefinition(Box<crate::datadogV1::model::ScatterPlotWidgetDefinition>),
    SunburstWidgetDefinition(Box<crate::datadogV1::model::SunburstWidgetDefinition>),
    TableWidgetDefinition(Box<crate::datadogV1::model::TableWidgetDefinition>),
    TimeseriesWidgetDefinition(Box<crate::datadogV1::model::TimeseriesWidgetDefinition>),
    ToplistWidgetDefinition(Box<crate::datadogV1::model::ToplistWidgetDefinition>),
    TreeMapWidgetDefinition(Box<crate::datadogV1::model::TreeMapWidgetDefinition>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SplitGraphSourceWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ChangeWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::ChangeWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GeomapWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::GeomapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::QueryValueWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::QueryValueWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ScatterPlotWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::ScatterPlotWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SunburstWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::SunburstWidgetDefinition(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TableWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::TableWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TimeseriesWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::TimeseriesWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ToplistWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::ToplistWidgetDefinition(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TreeMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SplitGraphSourceWidgetDefinition::TreeMapWidgetDefinition(
                    _v,
                ));
            }
        }

        return Ok(SplitGraphSourceWidgetDefinition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
