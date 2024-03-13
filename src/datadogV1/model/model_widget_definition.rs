// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// [Definition of the widget](<https://docs.datadoghq.com/dashboards/widgets/>).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WidgetDefinition {
    AlertGraphWidgetDefinition(Box<crate::datadogV1::model::AlertGraphWidgetDefinition>),
    AlertValueWidgetDefinition(Box<crate::datadogV1::model::AlertValueWidgetDefinition>),
    ChangeWidgetDefinition(Box<crate::datadogV1::model::ChangeWidgetDefinition>),
    CheckStatusWidgetDefinition(Box<crate::datadogV1::model::CheckStatusWidgetDefinition>),
    DistributionWidgetDefinition(Box<crate::datadogV1::model::DistributionWidgetDefinition>),
    EventStreamWidgetDefinition(Box<crate::datadogV1::model::EventStreamWidgetDefinition>),
    EventTimelineWidgetDefinition(Box<crate::datadogV1::model::EventTimelineWidgetDefinition>),
    FreeTextWidgetDefinition(Box<crate::datadogV1::model::FreeTextWidgetDefinition>),
    FunnelWidgetDefinition(Box<crate::datadogV1::model::FunnelWidgetDefinition>),
    GeomapWidgetDefinition(Box<crate::datadogV1::model::GeomapWidgetDefinition>),
    GroupWidgetDefinition(Box<crate::datadogV1::model::GroupWidgetDefinition>),
    HeatMapWidgetDefinition(Box<crate::datadogV1::model::HeatMapWidgetDefinition>),
    HostMapWidgetDefinition(Box<crate::datadogV1::model::HostMapWidgetDefinition>),
    IFrameWidgetDefinition(Box<crate::datadogV1::model::IFrameWidgetDefinition>),
    ImageWidgetDefinition(Box<crate::datadogV1::model::ImageWidgetDefinition>),
    ListStreamWidgetDefinition(Box<crate::datadogV1::model::ListStreamWidgetDefinition>),
    LogStreamWidgetDefinition(Box<crate::datadogV1::model::LogStreamWidgetDefinition>),
    MonitorSummaryWidgetDefinition(Box<crate::datadogV1::model::MonitorSummaryWidgetDefinition>),
    NoteWidgetDefinition(Box<crate::datadogV1::model::NoteWidgetDefinition>),
    PowerpackWidgetDefinition(Box<crate::datadogV1::model::PowerpackWidgetDefinition>),
    QueryValueWidgetDefinition(Box<crate::datadogV1::model::QueryValueWidgetDefinition>),
    RunWorkflowWidgetDefinition(Box<crate::datadogV1::model::RunWorkflowWidgetDefinition>),
    SLOListWidgetDefinition(Box<crate::datadogV1::model::SLOListWidgetDefinition>),
    SLOWidgetDefinition(Box<crate::datadogV1::model::SLOWidgetDefinition>),
    ScatterPlotWidgetDefinition(Box<crate::datadogV1::model::ScatterPlotWidgetDefinition>),
    ServiceMapWidgetDefinition(Box<crate::datadogV1::model::ServiceMapWidgetDefinition>),
    ServiceSummaryWidgetDefinition(Box<crate::datadogV1::model::ServiceSummaryWidgetDefinition>),
    SplitGraphWidgetDefinition(Box<crate::datadogV1::model::SplitGraphWidgetDefinition>),
    SunburstWidgetDefinition(Box<crate::datadogV1::model::SunburstWidgetDefinition>),
    TableWidgetDefinition(Box<crate::datadogV1::model::TableWidgetDefinition>),
    TimeseriesWidgetDefinition(Box<crate::datadogV1::model::TimeseriesWidgetDefinition>),
    ToplistWidgetDefinition(Box<crate::datadogV1::model::ToplistWidgetDefinition>),
    TopologyMapWidgetDefinition(Box<crate::datadogV1::model::TopologyMapWidgetDefinition>),
    TreeMapWidgetDefinition(Box<crate::datadogV1::model::TreeMapWidgetDefinition>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::AlertGraphWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::AlertGraphWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::AlertValueWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::AlertValueWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ChangeWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ChangeWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::CheckStatusWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::CheckStatusWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::DistributionWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::DistributionWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::EventStreamWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::EventStreamWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::EventTimelineWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::EventTimelineWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FreeTextWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::FreeTextWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::FunnelWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::FunnelWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GeomapWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::GeomapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::GroupWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::GroupWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HeatMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::HeatMapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HostMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::HostMapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::IFrameWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::IFrameWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::ImageWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ImageWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ListStreamWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ListStreamWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::LogStreamWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::LogStreamWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::MonitorSummaryWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::MonitorSummaryWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::NoteWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::NoteWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::PowerpackWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::PowerpackWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::QueryValueWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::QueryValueWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::RunWorkflowWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::RunWorkflowWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SLOListWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::SLOListWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SLOWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::SLOWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ScatterPlotWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ScatterPlotWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ServiceMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ServiceMapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ServiceSummaryWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ServiceSummaryWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SplitGraphWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::SplitGraphWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SunburstWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::SunburstWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::TableWidgetDefinition>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetDefinition::TableWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TimeseriesWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::TimeseriesWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::ToplistWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::ToplistWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TopologyMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::TopologyMapWidgetDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TreeMapWidgetDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetDefinition::TreeMapWidgetDefinition(_v));
            }
        }

        return Ok(WidgetDefinition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
