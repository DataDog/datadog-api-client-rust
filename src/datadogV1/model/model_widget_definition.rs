// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// [Definition of the widget](<https://docs.datadoghq.com/dashboards/widgets/>).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WidgetDefinition {
    AlertGraphWidgetDefinition(crate::datadogV1::model::AlertGraphWidgetDefinition),
    AlertValueWidgetDefinition(crate::datadogV1::model::AlertValueWidgetDefinition),
    ChangeWidgetDefinition(crate::datadogV1::model::ChangeWidgetDefinition),
    CheckStatusWidgetDefinition(crate::datadogV1::model::CheckStatusWidgetDefinition),
    DistributionWidgetDefinition(crate::datadogV1::model::DistributionWidgetDefinition),
    EventStreamWidgetDefinition(crate::datadogV1::model::EventStreamWidgetDefinition),
    EventTimelineWidgetDefinition(crate::datadogV1::model::EventTimelineWidgetDefinition),
    FreeTextWidgetDefinition(crate::datadogV1::model::FreeTextWidgetDefinition),
    FunnelWidgetDefinition(crate::datadogV1::model::FunnelWidgetDefinition),
    GeomapWidgetDefinition(crate::datadogV1::model::GeomapWidgetDefinition),
    GroupWidgetDefinition(crate::datadogV1::model::GroupWidgetDefinition),
    HeatMapWidgetDefinition(crate::datadogV1::model::HeatMapWidgetDefinition),
    HostMapWidgetDefinition(crate::datadogV1::model::HostMapWidgetDefinition),
    IFrameWidgetDefinition(crate::datadogV1::model::IFrameWidgetDefinition),
    ImageWidgetDefinition(crate::datadogV1::model::ImageWidgetDefinition),
    ListStreamWidgetDefinition(crate::datadogV1::model::ListStreamWidgetDefinition),
    LogStreamWidgetDefinition(crate::datadogV1::model::LogStreamWidgetDefinition),
    MonitorSummaryWidgetDefinition(crate::datadogV1::model::MonitorSummaryWidgetDefinition),
    NoteWidgetDefinition(crate::datadogV1::model::NoteWidgetDefinition),
    PowerpackWidgetDefinition(crate::datadogV1::model::PowerpackWidgetDefinition),
    QueryValueWidgetDefinition(crate::datadogV1::model::QueryValueWidgetDefinition),
    RunWorkflowWidgetDefinition(crate::datadogV1::model::RunWorkflowWidgetDefinition),
    SLOListWidgetDefinition(crate::datadogV1::model::SLOListWidgetDefinition),
    SLOWidgetDefinition(crate::datadogV1::model::SLOWidgetDefinition),
    ScatterPlotWidgetDefinition(crate::datadogV1::model::ScatterPlotWidgetDefinition),
    ServiceMapWidgetDefinition(crate::datadogV1::model::ServiceMapWidgetDefinition),
    ServiceSummaryWidgetDefinition(crate::datadogV1::model::ServiceSummaryWidgetDefinition),
    SplitGraphWidgetDefinition(crate::datadogV1::model::SplitGraphWidgetDefinition),
    SunburstWidgetDefinition(crate::datadogV1::model::SunburstWidgetDefinition),
    TableWidgetDefinition(crate::datadogV1::model::TableWidgetDefinition),
    TimeseriesWidgetDefinition(crate::datadogV1::model::TimeseriesWidgetDefinition),
    ToplistWidgetDefinition(crate::datadogV1::model::ToplistWidgetDefinition),
    TopologyMapWidgetDefinition(crate::datadogV1::model::TopologyMapWidgetDefinition),
    TreeMapWidgetDefinition(crate::datadogV1::model::TreeMapWidgetDefinition),
}
