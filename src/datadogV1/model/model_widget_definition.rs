// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// [Definition of the widget](<https://docs.datadoghq.com/dashboards/widgets/>).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}
